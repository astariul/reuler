use crate::utils;

/// Compute the first triangular number with over n divisors.
fn triangular_n_divisors(n: usize) -> usize {
    let mut triangular = 0;
    let mut i = 1;

    loop {
        // Compute the next triangular number
        triangular += i;

        // Find the divisors
        let divisors = utils::get_divisors(triangular);

        if divisors.len() > n {
            return triangular;
        }

        i += 1;
    }
}

/// Solve the problem #12 and return the solution.
pub fn solve() -> String {
    triangular_n_divisors(500).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(triangular_n_divisors(5), 28);
    }
}
