/// Check if a given number is pentagonal or not.
fn is_pentagonal(x: usize) -> bool {
    // To check if a number is pentagonal, we have to inverse : Pn = n(3n - 1) / 2
    // We have x = Pn -> x = n(3n - 1) / 2
    // -> 2x = 3n^2 - n
    // -> 3n^2 - n - 2x = 0
    // The solution to this equation is n = (1 + sqrt(1 + 24x)) / 6
    // (considering that n should be positive)
    // And since n should be an integer, we just have to check
    // that 1 + sqrt(1 + 24x) is divisible by 6
    let sqrt = ((1 + 24 * x) as f64).sqrt() as usize;
    if sqrt * sqrt != 1 + 24 * x {
        // Not a perfect square root
        return false;
    }
    (1 + sqrt) % 6 == 0
}

/// Check if a given number is hexagonal or not.
fn is_hexagonal(x: usize) -> bool {
    // To check if a number is hexagonal, we have to inverse : Hn = n(2n - 1)
    // We have x = Hn -> x = n(2n - 1)
    // 2n^2 - n - x = 0
    // The solution to this equation is n = (1 + sqrt(1 + 8x)) / 4
    // (considering that n should be positive)
    // And since n should be an integer, we just have to check
    // that 1 + sqrt(1 + 8x) is divisible by 4
    let sqrt = ((1 + 8 * x) as f64).sqrt() as usize;
    if sqrt * sqrt != 1 + 8 * x {
        // Not a perfect square root
        return false;
    }
    (1 + sqrt) % 4 == 0
}

/// Compute the next triangular number (from given n) that is also pentagonal
/// and hexagonal.
fn next_triangular_pentagonal_hexagonal(n: usize) -> usize {
    let mut n = n + 1;
    loop {
        let tn = n * (n + 1) / 2;
        if is_pentagonal(tn) && is_hexagonal(tn) {
            return tn;
        }
        
        n += 1;
    }
}

/// Solve the problem #45 and return the solution.
pub fn solve() -> String {
    next_triangular_pentagonal_hexagonal(285).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(next_triangular_pentagonal_hexagonal(280), 40755);
    }

    #[test]
    fn test_is_pentagonal_true() {
        assert!(is_pentagonal(1));
        assert!(is_pentagonal(5));
        assert!(is_pentagonal(12));
        assert!(is_pentagonal(22));
    }

    #[test]
    fn test_is_pentagonal_false() {
        assert!(!is_pentagonal(2));
        assert!(!is_pentagonal(3));
        assert!(!is_pentagonal(7));
        assert!(!is_pentagonal(60));
    }

    #[test]
    fn test_is_hexagonal_true() {
        assert!(is_hexagonal(1));
        assert!(is_hexagonal(6));
        assert!(is_hexagonal(15));
        assert!(is_hexagonal(28));
    }

    #[test]
    fn test_is_hexagonal_false() {
        assert!(!is_hexagonal(2));
        assert!(!is_hexagonal(3));
        assert!(!is_hexagonal(7));
        assert!(!is_hexagonal(60));
    }
}
