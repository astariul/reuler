use crate::utils;

/// Compute the sum of digits of the factorial of the given number.
fn factorial_digit_sum(x: usize) -> usize {
    // The number will be big, use a BigInt
    let mut number = utils::BigInt::from(1);

    for mul_factor in 2..x + 1 {
        number = number * mul_factor;
    }

    // Sum the digits of our big number
    number.digits.iter().sum()
}

/// Solve the problem #20 and return the solution.
pub fn solve() -> String {
    factorial_digit_sum(100).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(factorial_digit_sum(10), 27);
    }
}
