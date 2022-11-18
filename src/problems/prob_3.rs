/// Compute the largest prime factor of the given number.
fn largest_prime_factor(x: usize) -> usize {
    let mut n = x;
    let mut f = 2;

    while f * f <= n {
        if n % f == 0 {
            n /= f;
        } else {
            if f == 2 {
                f += 1;
            } else {
                f += 2;
            }
        }
    }
    n
}

/// Solve the problem #3 and return the solution.
pub fn solve() -> String {
    largest_prime_factor(600851475143).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}
