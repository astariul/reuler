use crate::utils;

/// Compute the sum of digit of the number elevated to the given power.
fn power_digit_sum(x: usize, p: usize) -> usize {
    // The number will be big, use a BigInt
    let mut number = utils::BigInt::from(x);

    for _ in 1..p {
        number = number * x;
    }

    // Sum the digits of our big number
    number.digits.iter().sum()
}

/// Solve the problem #16 and return the solution.
pub fn solve() -> String {
    power_digit_sum(2, 1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(power_digit_sum(2, 15), 26);
    }
}
