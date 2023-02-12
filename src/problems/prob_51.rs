use crate::utils;
use std::collections::HashSet;

fn replace_ith_digit(digits: &Vec<usize>, i: usize) -> Vec<usize> {
    let inversed_i = digits.len() - i - 1;
    let mut family = Vec::new();
    let start = if i == 0 { 1 } else { 0 };
    for d in start..10 {
        let mut member = digits.clone();
        member[inversed_i] = d;
        family.push(utils::digits_to_number(member));
    }
    family
}

fn replace_digit_i(digits: &Vec<usize>, i: usize) -> Vec<usize> {
    let mut family = Vec::new();
    let start = if digits[digits.len() - 1] == i { 1 } else { 0 };
    for d in start..10 {
        let member = digits
            .clone()
            .into_iter()
            .map(|m| if m == i { d } else { m })
            .collect();
        family.push(utils::digits_to_number(member));
    }
    family
}

fn replace_repeated_digits(digits: &Vec<usize>) -> Vec<Vec<usize>> {
    // First, find the repeated digits
    let mut repeated_digits = Vec::new();
    let mut seen_digits = HashSet::new();
    for d in digits.iter() {
        if seen_digits.contains(&d) {
            repeated_digits.push(d);
        } else {
            seen_digits.insert(d);
        }
    }

    let mut families = Vec::new();
    for &digit_to_replace in repeated_digits.into_iter() {
        families.push(replace_digit_i(digits, digit_to_replace));
    }
    families
}

/// Find the smallest prime where x digits can be replaced with the same digit,
/// yielding n primes out of the 10 generated numbers.
fn smallest_n_prime_replacement(n: usize) -> usize {
    if n > 10 {
        panic!("Can't generate {n} alternative numbers, there is only 10 digits.");
    }
    if n < 1 {
        panic!("Can't get the smallest number of a 0-family, it's empty. Specify a different n.");
    }

    // let mut seen = HashSet::new();
    for p in utils::Primes::new() {
        // Skip already seen primes
        // if seen.contains(&p) {
        //     continue;
        // }

        let p_digits = utils::digits_of(p);

        // Replace single digit, only in case of 6-family or less
        if n < 7 {
            for i in 0..p_digits.len() {
                let mut family = replace_ith_digit(&p_digits, i);
                family.retain(|&m| utils::is_prime(m));
                if family.len() == n {
                    return *family.iter().min().unwrap();
                }
            }
        }

        // Replace repeated digits
        let mut families = replace_repeated_digits(&p_digits);
        for family in families.iter_mut() {
            family.retain(|&m| utils::is_prime(m));
            if family.len() == n {
                return *family.iter().min().unwrap();
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
    fn test_wrong_input_too_big() {
        smallest_n_prime_replacement(25);
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_too_small() {
        smallest_n_prime_replacement(0);
    }
}
