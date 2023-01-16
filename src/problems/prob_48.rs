use crate::utils;

/// Find the last ten digits of the serie 1^1 + 2^2 + 3^3 + ... + n^n.
fn last_digits_of_self_powers_till(n: usize) -> usize {
    // Compute the sum
    let mut sum = utils::BigInt::new();
    for x in 1..n + 1 {
        let mut self_power = utils::BigInt::from(x);
        for _ in 1..x {
            self_power *= x;
        }
        sum = &sum + &self_power;
    }

    // Get the last 10 digits (since the digits are reversed, it's the first 10 elements of `sum.digits`)
    utils::digits_to_number(sum.digits.into_iter().take(10).collect())
}

/// Solve the problem #48 and return the solution.
pub fn solve() -> String {
    last_digits_of_self_powers_till(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(last_digits_of_self_powers_till(10), 405071317);
    }
}
