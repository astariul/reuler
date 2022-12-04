use crate::utils;
use std::collections::HashSet;

/// Compute the sum of all positive integers which cannot be written as the sum
/// of 2 abundant numbers.
fn non_abundant_sum() -> usize {
    // First, compute the abundant numbers until 28123 (it is proven that after
    // this limit, every numbers can be written as the sum of two abundant numbers)
    let mut abundant_numbers = HashSet::new();

    for n in 1..28123 + 1 {
        if utils::get_proper_divisors(n).iter().sum::<usize>() > n {
            abundant_numbers.insert(n);
        }
    }

    // Then, get the list of numbers that can't be written as a sum of abundant
    // numbers
    let mut non_abundant_numbers = Vec::new();
    for n in 1..28123 + 1 {
        let mut is_abundant_sum = false;
        for an in &abundant_numbers {
            if &n > an && abundant_numbers.contains(&(n - an)) {
                is_abundant_sum = true;
                break;
            }
        }

        if !is_abundant_sum {
            non_abundant_numbers.push(n);
        }
    }
    non_abundant_numbers.iter().sum()
}

/// Solve the problem #23 and return the solution.
pub fn solve() -> String {
    non_abundant_sum().to_string()
}
