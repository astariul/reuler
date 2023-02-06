use crate::utils;

/// Find the smallest prime where x digits can be replaced with the same digit,
/// yielding n primes out of the 10 generated numbers.
fn smallest_n_prime_replacement(n: usize) -> usize {
    
}

/// Solve the problem #51 and return the solution.
pub fn solve() -> String {
    smallest_n_prime_replacement(8).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_2() {
        assert_eq!(smallest_n_prime_replacement(6), 13);
    }

    #[test]
    fn test_given_example_2() {
        assert_eq!(smallest_n_prime_replacement(7), 56003);
    }
}
