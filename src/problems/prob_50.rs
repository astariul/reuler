use crate::utils;

/// Find the prime that can be written as a sum of prime, under the given limit
fn largest_consecutive_prime_sum_under(limit: usize) -> usize {
    // Use dynamic programming to keep track of several sums
    // sums[i][j] is the sum from prime index i to prime index i + j
    let mut sums: Vec<Vec<usize>> = Vec::new();
    for p in utils::Primes::new_up_to(limit) {
        // Update all previous sums
        for sum_i in sums.iter_mut() {
            sum_i.push(sum_i[sum_i.len() - 1] + p);
        }

        // Add the last one
        sums.push(vec![p]);
    }

    // Then, just find the sum that is prime and that has the biggest difference i-j
    let mut best_prime = 2;
    let mut best_size = 0;
    for sum_i in sums.iter() {
        for (j, s) in sum_i.iter().enumerate() {
            if j > best_size && s < &limit && utils::is_prime(*s) {
                best_size = j;
                best_prime = *s;
            }
        }
    }
    best_prime
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