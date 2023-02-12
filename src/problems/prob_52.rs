use crate::utils;
use std::collections::HashMap;

fn has_same_digits(ref_digits: &Vec<usize>, cmp_digits: &Vec<usize>) -> bool {
    if ref_digits.len() != cmp_digits.len() {
        return false;
    }

    let mut ref_counter = HashMap::new();
    for d in ref_digits {
        *ref_counter.entry(d).or_insert(0) += 1;
    }

    let mut cmp_counter = HashMap::new();
    for d in cmp_digits {
        *cmp_counter.entry(d).or_insert(0) += 1;
    }

    ref_counter == cmp_counter
}

/// Find the smallest positive integer x where x, 2x, ... nx contain the same
/// digits.
fn smallest_n_multiples_same_digits(n: usize) -> usize {
    if n > 9 {
        panic!("No number exists where x and {n}x have the same digits. Specify a smaller n.");
    }
    if n < 1 {
        panic!("n should be a positive integer (given : {n}).");
    }

    let mutliples: Vec<usize> = (2..n + 1).rev().collect();
    let mut x = 1;
    loop {
        let digits = utils::digits_of(x);
        if mutliples.iter().all(|&m| has_same_digits(&digits, &utils::digits_of(m * x))) {

            break;
        }
        x += 1;
    }
    x
}

/// Solve the problem #52 and return the solution.
pub fn solve() -> String {
    smallest_n_multiples_same_digits(6).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(smallest_n_multiples_same_digits(2), 125874);
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_too_big() {
        smallest_n_multiples_same_digits(10);
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_too_small() {
        smallest_n_multiples_same_digits(0);
    }

    #[test]
    fn test_has_same_digits_true() {
        let n_1 = utils::digits_of(123789);
        let n_2 = utils::digits_of(789321);
        assert!(has_same_digits(&n_1, &n_2));
    }

    #[test]
    fn test_has_same_digits_false_digits_different() {
        let n_1 = utils::digits_of(3);
        let n_2 = utils::digits_of(5);
        assert!(!has_same_digits(&n_1, &n_2));
    }

    #[test]
    fn test_has_same_digits_false_counts_different() {
        let n_1 = utils::digits_of(123456);
        let n_2 = utils::digits_of(1234456);
        assert!(!has_same_digits(&n_1, &n_2));
    }
}
