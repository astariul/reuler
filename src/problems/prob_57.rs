use crate::utils;

/// When expanding the square root up to the given number of expansions, find
/// the number of expansions that are "fat" : where the numerator has more
/// digits than the denominator.
///
/// # How it works
/// If we write the iterations recursively :
/// f(n+1) = 1 + 1 / (1 + f(n))
///
/// If we write this as a fraction : f(n) = a(n) / b(n)
///
/// f(n+1) = 1 + 1 / (1 + a(n) / b(n))
/// f(n+1) = 1 + 1 / ((b(n) + a(n)) / b(n))
/// f(n+1) = 1 + b(n) / (b(n) + a(n))
/// f(n+1) = (2 * b(n) + a(n)) / (b(n) + a(n))
///
/// So we have :
/// a(n+1) = 2 * b(n) + a(n)
/// b(n+1) = a(n) + b(n)
fn n_fat_square_root_expansions(n_expansions: usize) -> usize {
    let mut n_fat_expansions = 0;
    let mut a = utils::BigInt::from(1);
    let mut b = utils::BigInt::from(1);
    let mut expansion_i = 0;

    while expansion_i < n_expansions {
        let new_b = &a + &b;
        b *= 2;
        a = &b + &a;
        b = new_b;
        expansion_i += 1;

        if a.digits.len() > b.digits.len() {
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
