/// Possible ranks of a card.
enum Rank {
    Pip(usize),
    Jack,
    Queen,
    King,
    Ace,
}

/// Possible suits of a card.
#[derive(Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

/// Represents a single card, with a rank, a suit, and an associated value. 
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Parse the given card descriptor and create the associated card.
    pub fn new(c: &str) -> Self {
        assert_eq!(c.len(), 2);
        let rank = match c.chars().nth(0).unwrap() {
            '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Rank::Pip(c.chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap()),
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

/// Contains the different scores for each type of hand.
mod Score {
    const BASE: usize = 100;
    const HIGH_CARD: usize = 0 * BASE;
    const ONE_PAIR: usize = 1 * BASE;
    const TWO_PAIR: usize = 2 * BASE;
    const THREE_OF_A_KIND: usize = 3 * BASE;
    const STRAIGHT: usize = 4 * BASE;
    const FLUSH: usize = 5 * BASE;
    const FULL_HOUSE: usize = 6 * BASE;
    const FOUR_OF_A_KIND: usize = 7 * BASE;
    const STRAIGHT_FLUSH: usize = 8 * BASE;
    const ROYAL_FLUSH: usize = 9 * BASE;
}

/// Represents a hand, which is several cards (5 in this problem).
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Constructor for the Hand.
    pub fn new(cards: Vec<Card>) -> Self {
        if cards.len() == 0 {
            panic!("A hand cannot be empty !");
        }
        Self { cards }
    }

    /// Compute the value of the hand.
    /// It's in this function that we compute the various ranks of a Poker hand
    /// (Full House, Royal Flush, etc...). Ranks are given value in the hundred
    /// scale, and then the value of each card are used for solving ties of
    /// same rank.
    /// A vector of values is returned. It should be iterated while comparing,
    /// and only if the i-th value is a tie, we should look into the i+1-th
    /// value.
    pub fn value(&self) -> Vec<usize> {
        // TODO
        let mut total_value = 0;
        for card in self.cards.iter() {
            total_value += card.value();
        }
        vec![total_value]
    }

    /// Return `true` if all card of the hand belong to the same suit.
    fn is_same_suit(&self) -> bool {
        let first_suit = self.cards[0].suit;
        self.cards.iter().all(|c| c.suit == first_suit)
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

        let p1_hand = Hand::new(p1_cards);
        let p2_hand = Hand::new(p2_cards);

        if p1_hand.value() > p2_hand.value() {
            p1_n_wins += 1;
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
        let h = Hand::new(Vec::new());
    }

    #[test]
    fn test_hand_is_same_suit() {
        let h = Hand::new(vec![
            Card::new("2H"),
            Card::new("4H"),
            Card::new("JH"),
        ]);
        assert!(h.is_same_suit());

        let h = Hand::new(vec![
            Card::new("2H"),
            Card::new("4S"),
            Card::new("JH"),
        ]);
        assert!(!h.is_same_suit());
    }
}
