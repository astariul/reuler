use crate::utils;
use std::collections::HashMap;

// Compute all the possible arrangements of indexes in groups of n_index
fn arrangements_of_index(indexes: Vec<usize>, n_index: usize) -> Vec<Vec<usize>> {
    // Base case, just return indexes one by one
    if n_index == 1 {
        return indexes.into_iter().map(|i| vec![i]).collect();
    }

    // Recursive case
    let mut arrangements = Vec::new();
    for i in 0..indexes.len() {
        // Select the indice i and build the remaining recursively
        let idx = indexes[i];
        let last_indexes = indexes[i + 1..].to_vec();
        let mut sub_arrangements = arrangements_of_index(last_indexes, n_index - 1);
        sub_arrangements.iter_mut().for_each(|a| a.push(idx));
        arrangements.extend(sub_arrangements);
    }
    arrangements
}

fn create_possible_replacements(x: usize, arrangements_cache: HashMap<usize, Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
    let original_digits = utils::digits_of(x);

    let replacements = Vec::new();
    for i in 1..original_digits.len() {
        // Create the replacements where i digits are replaced
        // First, get the possible arrangements for this number of digits
        let arrangements = arrangements_cache.entry(i).or_insert(arrangements_of_index((0..original_digits.len()).collect(), i));
        
        for arr in arrangements {
            let mut curr_replacement = Vec::new();
            // For each arrangement, replace with all possible digits from 0 to 9
            for d in 0..9 {
                let mut digits = original_digits.clone();
                for digit_to_replace in arr {
                    digits[*digit_to_replace] = d;
                }
                curr_replacement.push(utils::digits_to_number(digits));
            }
            replacements.push(curr_replacement);
        }
    }
    replacements
}

/// Find the smallest prime where x digits can be replaced with the same digit,
/// yielding n primes out of the 10 generated numbers.
fn smallest_n_prime_replacement(n: usize) -> usize {
    if n > 10 {
        panic!("Can't generate {n} alternative numbers, there is only 10 digits.");
    }

    let mut arrangements_cache = HashMap::new();
    for p in utils::Primes::new() {
        for replacement in create_possible_replacements(p, arrangements_cache) {
            let mut n_primes = 0;
            for r in replacement {
                if utils::is_prime(r) {
                    n_primes += 1;
                }
            }

            if n_primes == n {
                let mut min_r = replacement[0];
                for r in replacement {
                    if r < min_r {
                        min_r = r;
                    }
                }
                return min_r;
            }
        }
    }
    panic!("Unreachable code");
}

/// Solve the problem #51 and return the solution.
pub fn solve() -> String {
    smallest_n_prime_replacement(8).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_1() {
        assert_eq!(smallest_n_prime_replacement(6), 13);
    }

    #[test]
    fn test_given_example_2() {
        assert_eq!(smallest_n_prime_replacement(7), 56003);
    }

    #[test]
    #[should_panic]
    fn test_wrong_input() {
        smallest_n_prime_replacement(25);
    }

    #[test]
    fn test_arrangements_of_index_base_case() {
        let arrangements = arrangements_of_index(vec![1, 2, 3], 1);
        assert_eq!(arrangements.len(), 3);
        assert!(arrangements.contains(&vec![1]));
        assert!(arrangements.contains(&vec![2]));
        assert!(arrangements.contains(&vec![3]));
    }

    #[test]
    fn test_arrangements_of_index_basic_case() {
        let arrangements = arrangements_of_index(vec![1, 2, 3, 4], 2);
        assert_eq!(arrangements.len(), 6);
        assert!(arrangements.contains(&vec![2, 1]));
        assert!(arrangements.contains(&vec![3, 1]));
        assert!(arrangements.contains(&vec![4, 1]));
        assert!(arrangements.contains(&vec![3, 2]));
        assert!(arrangements.contains(&vec![4, 2]));
        assert!(arrangements.contains(&vec![4, 3]));
    }
}
