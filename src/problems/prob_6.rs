/// Compute the difference between the sum of the squares of the first n
/// numbers and the square of the sum of the first n numbers.
fn sum_square_difference(x: usize) -> usize {
    let mut sum_squares = 0;
    let mut sum = 0;

    for i in 0..x + 1 {
        sum += i;
        sum_squares += i * i;
    }

    sum * sum - sum_squares
}

/// Solve the problem #6 and return the solution.
pub fn solve() -> String {
    sum_square_difference(100).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(sum_square_difference(10), 2640);
    }
}
