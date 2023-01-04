use std::collections::HashSet;

/// Compute the number of triangle words among the given list of words.
fn number_of_triangle_words(words: &str) -> usize {
    // For each word, compute the word value
    let mut word_values = Vec::new();
    let mut max_word_value = 0;
    for word in words.split(",") {
        let mut word_val = 0;
        for c in word.chars() {
            // Ignore "
            if c == '"' {
                continue;
            }

            word_val += (c as usize) - ('A' as usize) + 1;
        }
        word_values.push(word_val);

        // Keep track of the max word value, to know until where we have to compute triangle numbers
        if word_val > max_word_value {
            max_word_value = word_val;
        }
    }

    // Compute all triangle numbers up until we cover all words
    let mut triangle_numbers = HashSet::new();
    let mut n = 1;
    loop {
        let t = n * (n + 1) / 2;
        triangle_numbers.insert(t);

        if t >= max_word_value {
            break;
        }
        n += 1;
    }

    // Finally, count the number of words that are triangle numbers
    word_values
        .iter()
        .map(|v| triangle_numbers.contains(&v) as usize)
        .sum()
}

/// Solve the problem #42 and return the solution.
pub fn solve() -> String {
    let words = include_str!("data/words.txt");
    number_of_triangle_words(words).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(number_of_triangle_words("\"SKY\",\"SWAG\""), 1);
        assert_eq!(number_of_triangle_words("\"SKY\",\"SKY\""), 2);
        assert_eq!(number_of_triangle_words("\"SWAG\",\"SWAG\""), 0);
    }
}
