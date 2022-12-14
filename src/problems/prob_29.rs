use crate::utils;
use std::collections::HashSet;

/// Compute the number of different a^b, where a and b can be any number
/// between the given min and max.
fn distinct_powers(min: usize, max: usize) -> usize {
    let mut distincts = HashSet::new();

    for a in min..max + 1 {
        // First, compute the starting point
        let mut start = a;
        for _ in 1..min - 1 {
            start *= a;
        }

        // Then compute the numbers
        let mut curr = utils::BigInt::from(start);
        for _b in min..max + 1 {
            curr *= a;
            distincts.insert(curr.digits.clone());
        }
    }
    distincts.len()
}

/// Solve the problem #29 and return the solution.
pub fn solve() -> String {
    distinct_powers(2, 100).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(distinct_powers(2, 5), 15);
    }
}
