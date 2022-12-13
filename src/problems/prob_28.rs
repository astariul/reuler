/// Compute the sum of the numbers located on the diagonals of the spiral of
/// given size.
fn sum_spiral_diagonals(size: usize) -> usize {
    let mut sum = 1;
    let mut curr_size = 1;
    let mut curr_number = 1;

    while curr_size < size {
        curr_size += 2;
        for _ in 0..4 {
            curr_number += curr_size - 1;
            sum += curr_number;
        }
    }
    sum
}

/// Solve the problem #28 and return the solution.
pub fn solve() -> String {
    sum_spiral_diagonals(1001).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(sum_spiral_diagonals(5), 101);
    }
}
