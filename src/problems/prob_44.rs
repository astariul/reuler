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

/// Compute D such that D = |Pk - Pj|, where Pk, Pj, Pk + Pj, Pk - Pj are all
/// pentagonal numbers, and D is minimized.
fn minimal_pentagon_number() -> usize {
    let mut k = 1;
    loop {
        let pk = k * (3 * k - 1) / 2;
        for j in 1..k {
            let pj = j * (3 * j - 1) / 2;
            if is_pentagonal(pk - pj) && is_pentagonal(pk + pj) {
                return pk - pj;
            }
        }
        k += 1;
    }
}

/// Solve the problem #43 and return the solution.
pub fn solve() -> String {
    minimal_pentagon_number().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
