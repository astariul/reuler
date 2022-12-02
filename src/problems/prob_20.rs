/// Compute the sum of digits of the factorial of the given number.
fn factorial_digit_sum(x: usize) -> usize {
    let mut fact = 1;
    for i in 2..x + 1 {
        fact *= i;
    }

    // Then sum the digits
    let mut sum = 0;
    while fact != 0 {
        sum += fact % 10;
        fact = fact / 10;
    }
    sum
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
