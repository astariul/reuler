/// Compute the smallest number that is divisible by each of the numbers from
/// 1 to x.
fn smallest_multiple_up_to(x: usize) -> usize {
    if x < 2 {
        panic!("Invalid input ({x}), please provide a number higher than 1.");
    }

    let mut n = x;

    loop {
        let mut divisible_by_all = true;

        // Use `rev()` here to start from the high end and exit the loop earlier
        for i in (2..x).rev() {
            if n % i != 0 {
                divisible_by_all = false;
                break;
            }
        }

        if divisible_by_all {
            break;
        }

        // Go up by the highest number, to skip useless steps
        n += x;
    }
    n
}

/// Solve the problem #5 and return the solution.
pub fn solve() -> String {
    smallest_multiple_up_to(20).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(smallest_multiple_up_to(10), 2520);
    }

    #[test]
    #[should_panic]
    fn test_edge_case_0() {
        smallest_multiple_up_to(0);
    }

    #[test]
    #[should_panic]
    fn test_edge_case_1() {
        smallest_multiple_up_to(1);
    }
}
