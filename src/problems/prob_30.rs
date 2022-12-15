/// Compute the sum of the n-special numbers, where a n-special number is a
/// number that can be written as the sum of the n-th power of its digit.
fn sum_power_of_digits_nth(n: u32) -> usize {
    let mut sum = 0;

    // First, precompute the powers of each digit
    let mut powers = Vec::new();
    for i in 0..10_usize {
        powers.push(i.pow(n));
    }

    // Compute the upper limit
    let limit = 9_usize.pow(n + 1);

    // Then iterate over all digits, checking if they are a n-special number
    for d in 2..limit + 1 {
        // Get the digit power sum
        let mut remain = d;
        let mut d_power_sum = 0;
        while remain != 0 {
            d_power_sum += powers[remain % 10];
            remain = remain / 10;
        }

        // If it's a n-special number, add it to our sum
        if d_power_sum == d {
            sum += d;
        }
    }
    sum
}

/// Solve the problem #30 and return the solution.
pub fn solve() -> String {
    sum_power_of_digits_nth(5).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(sum_power_of_digits_nth(4), 19316);
    }
}
