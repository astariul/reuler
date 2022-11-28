/// Compute the sum of digit of the number elevated to the given power.
fn power_digit_sum(x: usize, p: usize) -> usize {
    // Store each digit in a vector that will grow over time
    let mut digits = vec![x];

    for _ in 1..p {
        // Multiply each digit by x, and don't forget to carry over !
        let mut carry_over = 0;
        for i in 0..digits.len() {
            let digit_result = digits[i] * x + carry_over;

            // Update current digit
            digits[i] = digit_result % 10;

            // Update carry over
            carry_over = digit_result / 10;
        }

        if carry_over != 0 {
            // We have a carry over left, add a new digit
            digits.push(carry_over);
        }
    }

    digits.iter().sum()
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
