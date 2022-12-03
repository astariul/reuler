/// Compute the sum of digits of the factorial of the given number.
fn factorial_digit_sum(x: usize) -> usize {
    // Store each digit in a vector that will grow over time
    let mut digits = vec![1];

    for mul_factor in 2..x + 1 {
        // Multiply each digit by x, and don't forget to carry over !
        let mut carry_over = 0;
        for i in 0..digits.len() {
            let digit_result = digits[i] * mul_factor + carry_over;

            // Update current digit
            digits[i] = digit_result % 10;

            // Update carry over
            carry_over = digit_result / 10;
        }

        while carry_over != 0 {
            // We have a carry over left, add a new digit
            let new_digit = carry_over % 10;
            digits.push(new_digit);

            carry_over = carry_over / 10;
        }
    }

    digits.iter().sum()
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
