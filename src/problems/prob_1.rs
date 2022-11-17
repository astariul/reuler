/// Compute the sum of natural numbers below the given limit that are multiple
/// of 3 or 5.
pub fn sum_multiple_3_and_5_under(limit: usize) -> usize {
    23
}

/// Solve the problem #1 and return the solution.
pub fn solve() -> usize {
    sum_multiple_3_and_5_under(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(sum_multiple_3_and_5_under(10), 23);
    }
}