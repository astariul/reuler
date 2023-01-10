use std::collections::HashMap;
use std::collections::HashSet;

/// Find the first n consecutive numbers that have n distinct prime factors.
fn distinct_prime_factors(n: usize) -> usize {
    let mut previous_primes = Vec::new();
    let mut x = 2;
    let mut prime_factors = HashMap::new();
    prime_factors.insert(1, HashSet::new());

    loop {
        // First, find the first factor among the previous primes
        let mut first_factor = None;
        for pp in previous_primes.iter() {
            if x % pp == 0 {
                first_factor = Some(pp);
                break;
            }
        }

        // Then find the other factors from our cache
        let factors = match first_factor {
            Some(ff) => {
                // Reduce x with the first factor
                let mut x_reduced = x / ff;
                while x_reduced % ff == 0 {
                    x_reduced /= ff;
                }

                // Find the prime factors of the reduced x and add it our first factor
                let mut factors = prime_factors.get(&x_reduced).unwrap().clone();
                factors.insert(*ff);
                factors
            }
            None => {
                // x is a prime number, the only factor is itself
                let mut factors = HashSet::new();
                factors.insert(x);
                factors
            }
        };

        // Update our caches
        prime_factors.insert(x, factors);
        if first_factor.is_none() {
            previous_primes.push(x);
        }

        // Check if the n previous consecutive numbers have n distinct prime factors
        let mut consecutive_numbers_have_n_factors = true;
        for i in 0..n {
            if prime_factors.get(&(x - i)).unwrap().len() != n {
                consecutive_numbers_have_n_factors = false;
                break;
            }
        }
        if consecutive_numbers_have_n_factors {
            break;
        }

        // Next iteration
        x += 1;
    }
    x - n + 1
}

/// Solve the problem #47 and return the solution.
pub fn solve() -> String {
    distinct_prime_factors(4).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example_n2() {
        assert_eq!(distinct_prime_factors(2), 14);
    }

    #[test]
    fn test_given_example_n3() {
        assert_eq!(distinct_prime_factors(3), 644);
    }
}
