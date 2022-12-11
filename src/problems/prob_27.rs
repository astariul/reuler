/// Compute the size of the given quadratic formula. The size of the formula is
/// the maximum values of n for consecutive values of n which gives primes.
fn formula_size(a: isize, b: isize) -> usize {
    let mut n = 0;

    loop {
        break;
    }
    n
}

/// Compute a * b where n^2 + an + b is a quadratic formula that gives the most
/// primes among all quadratic formulas for a, b <= limit.
fn max_quadratic_formula(limit: usize) -> isize {
    let mut max_size = 0;
    let mut best_a = 0;
    let mut best_b = 0;
    let limit = isize::try_from(limit).unwrap();

    for a in -limit..limit + 1 {
        for b in -limit..limit + 1 {
            let size = formula_size(a, b);

            if size > max_size {
                max_size = size;
                best_a = a;
                best_b = b;
            }
        }
    }
    best_a * best_b
}

/// Solve the problem #27 and return the solution.
pub fn solve() -> String {
    max_quadratic_formula(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(max_quadratic_formula(41), 41);
    }

    #[test]
    fn test_formula_size_1() {
        assert_eq!(formula_size(1, 41), 40);
    }

    #[test]
    fn test_formula_size_2() {
        assert_eq!(formula_size(-79, 1601), 80);
    }
}
