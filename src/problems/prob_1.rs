/// Compute the sum of natural numbers below the given limit that are multiple
/// of 3 or 5.
fn sum_multiple_3_and_5_under(limit: usize) -> usize {
    let mut sum = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

/// Solve the problem #1 and return the solution.
pub fn solve() -> String {
    sum_multiple_3_and_5_under(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(sum_multiple_3_and_5_under(10), 23);
    }
}
