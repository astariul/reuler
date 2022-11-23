use crate::utils;

/// Compute the sum of the primes under the given limit.
fn primes_sum(limit: usize) -> usize {
    let primes = utils::Primes::new_up_to(limit);
    let mut sum = 0;

    for p in primes {
        sum += p;
    }
    sum
}

/// Solve the problem #10 and return the solution.
pub fn solve() -> String {
    primes_sum(2000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(primes_sum(10), 17);
    }
}
