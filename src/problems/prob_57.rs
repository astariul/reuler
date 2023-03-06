use crate::utils;

/// When expanding the square root up to the given number of expansions, find
/// the number of expansions that are "fat" : where the numerator has more
/// digits than the denominator.
fn n_fat_square_root_expansions(n_expansions: usize) -> usize {
    let mut n_fat_expansions = 0;
    let mut curr_expansion = 1.0;
    let mut expansion_idx = 0;

    let mut denominator = 2;
    while expansion_idx < n_expansions {
        curr_expansion = 1.0 + (1.0 / (1.0 + curr_expansion));
        expansion_idx += 1;

        while (denominator as f64 * curr_expansion).fract() > 0.0 {
            denominator += 1;
        }
        let numerator = (denominator as f64 * curr_expansion) as usize;

        if utils::BigInt::from(numerator).digits.len() > utils::BigInt::from(denominator).digits.len() {
            n_fat_expansions += 1;
        }
    }
    n_fat_expansions
}

/// Solve the problem #57 and return the solution.
pub fn solve() -> String {
    n_fat_square_root_expansions(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(n_fat_square_root_expansions(10), 1);
    }
}
