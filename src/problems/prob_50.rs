use crate::utils;

/// Find the prime that can be written as a sum of prime, under the given limit
fn largest_consecutive_prime_sum_under(limit: usize) -> usize {
    let primes: Vec<usize> = utils::Primes::new_up_to(limit).collect();

    let mut best_sum = 0;
    let mut best_size = 0;

    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            sum += primes[j];

            if sum > limit {
                break;
            }

            if j - i > best_size && utils::is_prime(sum) {
                best_sum = sum;
                best_size = j - i;
            }
        }
    }
    best_sum
}

/// Solve the problem #50 and return the solution.
pub fn solve() -> String {
    largest_consecutive_prime_sum_under(1000000).to_string()
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