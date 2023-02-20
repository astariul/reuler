use std::collections::HashMap;

struct CachedLimitedCombinations {
    limit: usize,
    cache: HashMap<(usize, usize), usize>,
}

impl CachedLimitedCombinations {
    fn new(limit: usize) -> Self {
        let cache = HashMap::new();
        Self {
            limit: limit,
            cache: cache,
        }
    }

    fn of(&mut self, n: usize, r: usize) -> usize {
        if r == 0 || r == n {
            return 1;
        }
        if !self.cache.contains_key(&(n, r)) {
            let mut c = self.of(n - 1, r - 1) + self.of(n - 1, r);
            if c > self.limit {
                c = self.limit + 1
            }
            self.cache.insert((n, r), c);
        }
        self.cache[&(n, r)]
    }
}

/// Find the number of combinatorics selections where (n r) > above_x with any
/// r, and 1 <= n <= max_n.
fn combinatorics_selections_above(max_n: usize, above_x: usize) -> usize {
    let mut combinations = CachedLimitedCombinations::new(above_x);

    let mut n_exceeding_value = 0;
    for n in 1..max_n + 1 {
        for r in 1..n + 1 {
            if combinations.of(n, r) > above_x {
                n_exceeding_value += 1;
            }
        }
    }
    n_exceeding_value
}

/// Solve the problem #53 and return the solution.
pub fn solve() -> String {
    combinatorics_selections_above(100, 1000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(combinatorics_selections_above(23, 1000000), 4);
    }

    #[test]
    fn test_combinations_no_limit() {
        let mut combinations = CachedLimitedCombinations::new(1000);
        assert_eq!(combinations.of(5, 3), 10);
    }

    #[test]
    fn test_combinations_with_limit() {
        let mut combinations = CachedLimitedCombinations::new(5);
        assert_eq!(combinations.of(5, 3), 6);
    }
}
