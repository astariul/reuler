use crate::utils;

/// Find the prime that can be written as a sum of prime, under the given limit
fn largest_consecutive_prime_sum_under(limit: usize) -> usize {
    let mut old_primes = Vec::new();
    for p in utils::Primes::new_up_to(limit) {
        //
    }
}

/// Solve the problem #50 and return the solution.
pub fn solve() -> String {
    largest_consecutive_prime_sum_under().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_1() {
        assert_eq!(largest_consecutive_prime_sum_under(100), 41);
    }

    #[test]
    fn test_given_example_2() {
        assert_eq!(largest_consecutive_prime_sum_under(1000), 953);
    }
}