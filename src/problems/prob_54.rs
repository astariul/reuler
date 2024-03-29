/// Possible ranks of a card.
#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Pip(usize),
    Jack,
    Queen,
    King,
    Ace,
}

/// Possible suits of a card.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

/// Represents a single card, with a rank, a suit, and an associated value.
#[derive(Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Parse the given card descriptor and create the associated card.
    pub fn new(c: &str) -> Self {
        assert_eq!(c.len(), 2);
        let rank = match c.chars().nth(0).unwrap() {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Rank::Pip(
                c.chars()
                    .nth(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
            'T' => Rank::Pip(10),
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => panic!("Unknown card rank : {}", c.chars().nth(0).unwrap()),
        };
        let suit = match c.chars().nth(1).unwrap() {
            'C' => Suit::Club,
            'D' => Suit::Diamond,
            'H' => Suit::Heart,
            'S' => Suit::Spade,
            _ => panic!("Unknown card suit : {}", c.chars().nth(1).unwrap()),
        };
        Self { rank, suit }
    }

    /// Compute the value of this card.
    pub fn value(&self) -> usize {
        match self.rank {
            Rank::Pip(x) => x,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.rank {
                Rank::Pip(r) => r.to_string(),
                Rank::Jack => "J".to_string(),
                Rank::Queen => "Q".to_string(),
                Rank::King => "K".to_string(),
                Rank::Ace => "A".to_string(),
            },
            match self.suit {
                Suit::Club => "♧",
                Suit::Diamond => "♢",
                Suit::Heart => "♡",
                Suit::Spade => "♤",
            }
        )
    }
}

/// Contains the different scores for each type of hand.
mod score {
    const _BASE: usize = 1000;
    pub const PAIR: usize = 1 * _BASE;
    pub const TWO_PAIRS: usize = 2 * _BASE;
    pub const THREE_OF_A_KIND: usize = 3 * _BASE;
    pub const STRAIGHT: usize = 4 * _BASE;
    pub const FLUSH: usize = 5 * _BASE;
    pub const FULL_HOUSE: usize = 6 * _BASE;
    pub const FOUR_OF_A_KIND: usize = 7 * _BASE;
    pub const STRAIGHT_FLUSH: usize = 8 * _BASE;
    pub const ROYAL_FLUSH: usize = 9 * _BASE;
}

/// Represents a hand, which is several cards (5 in this problem).
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Constructor for the Hand.
    pub fn new(mut cards: Vec<Card>) -> Self {
        if cards.len() == 0 {
            panic!("A hand cannot be empty !");
        }
        cards.sort_by_key(|c| c.value());
        Self { cards }
    }

    /// Compute the values of the hand.
    /// It's in this function that we compute the various ranks of a Poker hand
    /// (Full House, Royal Flush, etc...). Ranks are given value in the
    /// thousand scale, and then the value of each card are used for solving
    /// ties of same rank.
    /// A vector of values is returned. It should be iterated while comparing,
    /// and only if the i-th value is a tie, we should look into the i+1-th
    /// value.
    pub fn values(&self) -> Vec<usize> {
        // Royal Flush
        if self.is_same_suit()
            && self.is_consecutive()
            && self.cards.len() == 5
            && self.cards[0].rank == Rank::Pip(10)
        {
            // All cards are used and all Royal flushes are equal, so no need to update the score for tie-solving
            return vec![score::ROYAL_FLUSH];
        }

        // Straight Flush
        if self.is_same_suit() && self.is_consecutive() {
            // All cards are used so there is only one value
            // But the score of the straight flush should be updated with the value of the highest card for tie-solving
            return vec![score::STRAIGHT_FLUSH + self.cards[self.cards.len() - 1].value()];
        }

        // From here we need to count the cards with the same value
        // Group the cards by value
        let mut value_groups: Vec<Vec<Card>> = Vec::new();
        for c in self.cards.iter() {
            let mut added_to_existing_group = false;
            for group in value_groups.iter_mut() {
                if group[0].value() == c.value() {
                    group.push(c.clone());
                    added_to_existing_group = true;
                }
            }

            if !added_to_existing_group {
                value_groups.push(vec![c.clone()]);
            }
        }
        // Order groups based on how many cards are in the group and the value of the group
        value_groups.sort_by_key(|g| std::cmp::Reverse(g.len() * 100 + g[0].value()));

        // Four of a kind
        if value_groups[0].len() == 4 {
            // Tie-solving for the quadruplet : value of the group
            let mut values = vec![score::FOUR_OF_A_KIND + value_groups[0][0].value()];

            // For the other groups, just the value of the group
            for i in 1..value_groups.len() {
                values.push(value_groups[i][0].value());
            }
            return values;
        }

        // Full house
        if value_groups.len() >= 2 && value_groups[0].len() == 3 && value_groups[1].len() == 2 {
            // Tie-solving for the triplet & pair : value of the groups
            // The value of the triplet is more important though, hence the weighting
            let mut values = vec![
                score::FULL_HOUSE + value_groups[0][0].value() * 10 + value_groups[1][0].value(),
            ];

            // For the other groups, just the value of the group
            for i in 2..value_groups.len() {
                values.push(value_groups[i][0].value());
            }
            return values;
        }

        // Flush
        if self.is_same_suit() {
            // All cards are used so there is only one value
            return vec![score::FLUSH + self.cards[self.cards.len() - 1].value()];
        }

        // Straight
        if self.is_consecutive() {
            // All cards are used so there is only one value
            return vec![score::STRAIGHT + self.cards[self.cards.len() - 1].value()];
        }

        // Three of a kind
        if value_groups[0].len() == 3 {
            // Tie-solving for the triplet : value of the group
            let mut values = vec![score::THREE_OF_A_KIND + value_groups[0][0].value()];

            // For the other groups, just the value of the group
            for i in 1..value_groups.len() {
                values.push(value_groups[i][0].value());
            }
            return values;
        }

        // Two pairs
        if value_groups.len() >= 2 && value_groups[0].len() == 2 && value_groups[1].len() == 2 {
            // Tie-solving for the double pair : value of the groups
            let mut values =
                vec![score::TWO_PAIRS + value_groups[0][0].value() + value_groups[1][0].value()];

            // For the other groups, just the value of the group
            for i in 2..value_groups.len() {
                values.push(value_groups[i][0].value());
            }
            return values;
        }

        // Pair
        if value_groups[0].len() == 2 {
            // Tie-solving for the pair : value of the groups
            let mut values = vec![score::PAIR + value_groups[0][0].value()];

            // For the other groups, just the value of the group
            for i in 1..value_groups.len() {
                values.push(value_groups[i][0].value());
            }
            return values;
        }

        // Highest card
        self.cards.iter().rev().map(|c| c.value()).collect()
    }

    /// Return `true` if all cards of the hand belong to the same suit.
    fn is_same_suit(&self) -> bool {
        let first_suit = self.cards[0].suit;
        self.cards.iter().all(|c| c.suit == first_suit)
    }

    /// Return `true` if the cards of the hands are consecutive.
    fn is_consecutive(&self) -> bool {
        for i in 1..self.cards.len() {
            if self.cards[i].value() != self.cards[i - 1].value() + 1 {
                return false;
            }
        }
        true
    }
}

/// Find the number of time where player 1 wins, given a list of 2 hands with
/// each 5 cards.
fn n_win_player_1(hands: &str) -> usize {
    let mut p1_n_wins = 0;
    for hand in hands.lines() {
        let mut cards = Vec::new();
        for card_descriptor in hand.split_whitespace() {
            cards.push(Card::new(card_descriptor));
        }
        let p2_cards = cards.split_off(cards.len() / 2);
        let p1_cards = cards;

        let p1_hand_values = Hand::new(p1_cards).values();
        let p2_hand_values = Hand::new(p2_cards).values();

        for i in 0..p1_hand_values.len() {
            if p1_hand_values[i] > p2_hand_values[i] {
                p1_n_wins += 1;
            }

            // Only keep going in this loop if there is a tie
            if p1_hand_values[i] != p2_hand_values[i] {
                break;
            }
        }
    }
    p1_n_wins
}

/// Solve the problem #54 and return the solution.
pub fn solve() -> String {
    let poker_hands = include_str!("data/poker.txt");
    n_win_player_1(poker_hands).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_1() {
        assert_eq!(n_win_player_1("5H 5C 6S 7S KD 2C 3S 8S 8D TD"), 0);
    }

    #[test]
    fn test_given_example_2() {
        assert_eq!(n_win_player_1("5D 8C 9S JS AC 2C 5C 7D 8S QH"), 1);
    }

    #[test]
    fn test_given_example_3() {
        assert_eq!(n_win_player_1("2D 9C AS AH AC 3D 6D 7D TD QD"), 0);
    }

    #[test]
    fn test_given_example_4() {
        assert_eq!(n_win_player_1("4D 6S 9H QH QC 3D 6D 7H QD QS"), 1);
    }

    #[test]
    fn test_given_example_5() {
        assert_eq!(n_win_player_1("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D"), 1);
    }

    #[test]
    #[should_panic]
    fn test_build_card_wrong_rank_1() {
        Card::new("1H");
    }

    #[test]
    #[should_panic]
    fn test_build_card_wrong_rank_x() {
        Card::new("XH");
    }

    #[test]
    #[should_panic]
    fn test_build_card_wrong_suit() {
        Card::new("4X");
    }

    #[test]
    fn test_card_values() {
        assert!(Card::new("2H").value() < Card::new("3H").value());
        assert!(Card::new("3H").value() < Card::new("4H").value());
        assert!(Card::new("4H").value() < Card::new("5H").value());
        assert!(Card::new("5H").value() < Card::new("6H").value());
        assert!(Card::new("6H").value() < Card::new("7H").value());
        assert!(Card::new("7H").value() < Card::new("8H").value());
        assert!(Card::new("8H").value() < Card::new("9H").value());
        assert!(Card::new("9H").value() < Card::new("TH").value());
        assert!(Card::new("TH").value() < Card::new("JH").value());
        assert!(Card::new("JH").value() < Card::new("QH").value());
        assert!(Card::new("QH").value() < Card::new("KH").value());
        assert!(Card::new("KH").value() < Card::new("AH").value());
    }

    #[test]
    #[should_panic]
    fn test_empty_hand() {
        Hand::new(Vec::new());
    }

    #[test]
    fn test_hand_is_same_suit() {
        let h = Hand::new(vec![Card::new("2H"), Card::new("4H"), Card::new("JH")]);
        assert!(h.is_same_suit());

        let h = Hand::new(vec![Card::new("2H"), Card::new("4S"), Card::new("JH")]);
        assert!(!h.is_same_suit());
    }

    #[test]
    fn test_hand_ordered() {
        let h = Hand::new(vec![
            Card::new("JH"),
            Card::new("2S"),
            Card::new("AD"),
            Card::new("4C"),
        ]);
        assert_eq!(h.cards[0].suit, Suit::Spade);
        assert_eq!(h.cards[1].suit, Suit::Club);
        assert_eq!(h.cards[2].suit, Suit::Heart);
        assert_eq!(h.cards[3].suit, Suit::Diamond);
    }

    #[test]
    fn test_hand_is_consecutive() {
        let h = Hand::new(vec![Card::new("2H"), Card::new("4S"), Card::new("JH")]);
        assert!(!h.is_consecutive());

        let h = Hand::new(vec![Card::new("2H"), Card::new("3S"), Card::new("4H")]);
        assert!(h.is_consecutive());

        let h = Hand::new(vec![
            Card::new("9H"),
            Card::new("TS"),
            Card::new("JH"),
            Card::new("QS"),
        ]);
        assert!(h.is_consecutive());
    }

    #[test]
    fn test_score_royal_flush() {
        let values = Hand::new(vec![
            Card::new("TH"),
            Card::new("JH"),
            Card::new("QH"),
            Card::new("KH"),
            Card::new("AH"),
        ])
        .values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], score::ROYAL_FLUSH);
    }

    #[test]
    fn test_score_straight_flush() {
        let values = Hand::new(vec![
            Card::new("7H"),
            Card::new("8H"),
            Card::new("9H"),
            Card::new("TH"),
            Card::new("JH"),
        ])
        .values();
        assert_eq!(values.len(), 1);
        assert!(values[0] > score::STRAIGHT_FLUSH);
    }

    #[test]
    fn test_score_four_of_a_kind() {
        let values = Hand::new(vec![
            Card::new("4H"),
            Card::new("6C"),
            Card::new("6S"),
            Card::new("6D"),
            Card::new("6H"),
        ])
        .values();
        assert_eq!(values.len(), 2);
        assert!(values[0] > score::FOUR_OF_A_KIND);
        assert_eq!(values[1], 4);
    }

    #[test]
    fn test_score_full_house() {
        let values = Hand::new(vec![
            Card::new("2H"),
            Card::new("2S"),
            Card::new("2D"),
            Card::new("6D"),
            Card::new("6H"),
        ])
        .values();
        assert_eq!(values.len(), 1);
        assert!(values[0] > score::FULL_HOUSE);
    }

    #[test]
    fn test_score_flush() {
        let values = Hand::new(vec![
            Card::new("2H"),
            Card::new("4H"),
            Card::new("6H"),
            Card::new("8H"),
            Card::new("QH"),
        ])
        .values();
        assert_eq!(values.len(), 1);
        assert!(values[0] > score::FLUSH);
    }

    #[test]
    fn test_score_straight() {
        let values = Hand::new(vec![
            Card::new("2H"),
            Card::new("3S"),
            Card::new("4D"),
            Card::new("5H"),
            Card::new("6H"),
        ])
        .values();
        assert_eq!(values.len(), 1);
        assert!(values[0] > score::STRAIGHT);
    }

    #[test]
    fn test_score_three_of_a_kind() {
        let values = Hand::new(vec![
            Card::new("2H"),
            Card::new("3S"),
            Card::new("6S"),
            Card::new("6D"),
            Card::new("6H"),
        ])
        .values();
        assert_eq!(values.len(), 3);
        assert!(values[0] > score::THREE_OF_A_KIND);
        assert_eq!(values[1], 3);
        assert_eq!(values[2], 2);
    }

    #[test]
    fn test_score_two_pairs() {
        let values = Hand::new(vec![
            Card::new("8H"),
            Card::new("3S"),
            Card::new("8S"),
            Card::new("6D"),
            Card::new("6H"),
        ])
        .values();
        assert_eq!(values.len(), 2);
        assert!(values[0] > score::TWO_PAIRS);
        assert_eq!(values[1], 3);
    }

    #[test]
    fn test_score_pair() {
        let values = Hand::new(vec![
            Card::new("8H"),
            Card::new("3S"),
            Card::new("8S"),
            Card::new("9D"),
            Card::new("KH"),
        ])
        .values();
        assert_eq!(values.len(), 4);
        assert!(values[0] > score::PAIR);
        assert_eq!(values[1], 13);
        assert_eq!(values[2], 9);
        assert_eq!(values[3], 3);
    }

    #[test]
    fn test_score_high_card() {
        let values = Hand::new(vec![
            Card::new("8H"),
            Card::new("3S"),
            Card::new("5S"),
            Card::new("9D"),
            Card::new("KH"),
        ])
        .values();
        assert_eq!(values.len(), 5);
        assert_eq!(values[0], 13);
        assert_eq!(values[1], 9);
        assert_eq!(values[2], 8);
        assert_eq!(values[3], 5);
        assert_eq!(values[4], 3);
    }
}
