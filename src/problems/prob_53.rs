use crate::utils;
use std::collections::HashMap;

struct CachedFactorial {
    cache: HashMap<usize, utils::BigInt>,
}

impl CachedFactorial {
    fn new() -> Self {
        let mut cache = HashMap::new();
        cache.insert(0, utils::BigInt::from(1));
        Self { cache }
    }

    fn of(&self, n: usize) -> utils::BigInt {
        if !self.cache.contains_key(&n) {
            let mut f = self.of(n - 1);
            f *= n;
            self.cache.insert(n, f);
        }
        self.cache[&n]
    }
}

/// Find the number of combinatorics selections where (n r) has more than
/// n_digits digits with any r, and 1 <= n <= max_n
fn combinatorics_selections_above(max_n: usize, n_digits: usize) -> usize {
    let factorial = CachedFactorial::new();

    let mut n_exceeding_value = 0;
    for n in 1..max_n + 1 {
        for r in 1..n + 1 {
            let x = factorial.of(n) / (factorial.of(r) * factorial.of(n - r));
            if x.digits.len() > n_digits {
                n_exceeding_value += 1;
            }
        }
    }
    n_exceeding_value
}

/// Solve the problem #53 and return the solution.
pub fn solve() -> String {
    combinatorics_selections_above(100, 6).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(combinatorics_selections_above(23, 6), 4);
    }
}
