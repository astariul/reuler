use crate::utils;

/// Check if a given number is a truncatable prime.
fn is_truncatable_prime(x: usize) -> bool {
    // Single digits numbers are never truncatable primes
    if x < 10 {
        return false;
    }

    // From right to left
    let mut n = x;
    while n != 0 {
        if !utils::is_prime(n) {
            return false;
        }
        n /= 10;
    }

    // From left to right
    let mut m = 10;
    while m < x {
        if !utils::is_prime(x % m) {
            return false;
        }
        m *= 10;
    }

    // If everything is prime, then it's a truncatable prime
    true
}

/// Compute the sum of all truncatable primes.
fn sum_truncatable_primes() -> usize {
    let mut sum = 0;
    let mut n_truncatable_primes = 0;

    for p in utils::Primes::new() {
        if is_truncatable_prime(p) {
            sum += p;
            n_truncatable_primes += 1;
        }

        // We know there is only 11 truncatable primes
        if n_truncatable_primes == 11 {
            break;
        }
    }
    sum
}

/// Solve the problem #37 and return the solution.
pub fn solve() -> String {
    sum_truncatable_primes().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truncatable_prime_true() {
        assert!(is_truncatable_prime(3797));
    }

    #[test]
    fn test_is_truncatable_prime_false_prime() {
        assert!(!is_truncatable_prime(4493));
    }

    #[test]
    fn test_is_truncatable_prime_false_non_prime() {
        assert!(!is_truncatable_prime(333));
    }

    #[test]
    fn test_is_truncatable_prime_false_single_digit_prime() {
        assert!(!is_truncatable_prime(5));
    }
}
