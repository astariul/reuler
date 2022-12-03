use std::collections::HashSet;
use crate::utils;


/// Compute the d() function as defined in the problem : the sum of proper
/// divisors of n.
fn d(n: usize) -> usize {
    let mut divisors = utils::get_divisors(n);
    divisors.remove(&n);     // We want the proper divisors, not just the divisors
    divisors.iter().sum()
}


/// Compute the sum of all amicable numbers under the given limit.
fn sum_amicable_numbers_under(limit: usize) -> usize {
    let mut amicable_n = HashSet::new();

    for n in 1..limit {
        // If n is amicable, it's done, skip it
        if amicable_n.contains(&n) {
            continue;
        }

        // Otherwise, let's see if it's amicable
        let dx = d(n);
        if dx != n && d(dx) == n {
            amicable_n.insert(n);
            amicable_n.insert(dx);
        }
    }
    amicable_n.iter().sum()
}

/// Solve the problem #21 and return the solution.
pub fn solve() -> String {
    sum_amicable_numbers_under(10000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_amicable() {
        assert_eq!(d(220), 284);
        assert_eq!(d(284), 220);
    }

    #[test]
    fn test_example() {
        assert_eq!(sum_amicable_numbers_under(300), 504);
    }
}
