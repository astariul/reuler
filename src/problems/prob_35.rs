use crate::utils;

fn is_circular_prime(x: usize) -> bool {
    // First, get the digits of the number
    let mut digits = Vec::new();
    let mut remain = x;
    while remain > 0 {
        digits.push(remain % 10);
        remain = remain / 10;
    }

    // Then rotate it on every possible position and ensure each produced position is prime
    for _ in 0..digits.len() {
        // Recreate the number
        let mut n = 0;
        for (i, v) in digits.iter().enumerate() {
            n += v * 10_usize.pow(i.try_into().unwrap());
        }

        // Check if it's prime
        if !utils::is_prime(n) {
            return false;
        }

        // Rotate the digits for the next iteration
        digits.rotate_right(1);
    }
    true
}

/// Compute the number of circular prime numbers under a given limit.
fn n_circular_prime_below(limit: usize) -> usize {
    let mut n_circular_primes = 0;
    let primes = utils::Primes::new_up_to(limit);

    for p in primes {
        if is_circular_prime(p) {
            n_circular_primes += 1;
        }
    }
    n_circular_primes
}

/// Solve the problem #35 and return the solution.
pub fn solve() -> String {
    n_circular_prime_below(1000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(n_circular_prime_below(100), 13);
    }

    #[test]
    fn test_is_circular_prime_true() {
        assert!(is_circular_prime(17));
    }

    #[test]
    fn test_is_circular_prime_false() {
        assert!(!is_circular_prime(19));
    }

    #[test]
    fn test_is_circular_prime_not_prime() {
        assert!(!is_circular_prime(54));
    }
}
