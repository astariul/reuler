use crate::utils;


/// Compute the n-th prime number.
fn nth_prime(n: usize) -> usize {
    let mut primes = utils::Primes::new();

    let mut p = 1;
    for _i in 0..n {
        p = primes.next().unwrap();
    }
    p
}

/// Solve the problem #7 and return the solution.
pub fn solve() -> String {
    nth_prime(10001).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(nth_prime(6), 13);
    }
}
