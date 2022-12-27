use crate::utils;

/// Check if a given number is a double palindrome
fn is_double_base_palindrome(x: usize) -> bool {
    // First, get the digits of the number in base 10
    let digits = utils::digits_of_base(x, 10);

    // Check if it's a palindrome
    if !utils::is_palindrome(digits) {
        return false;
    }

    // Then do the same for base 2
    let digits = utils::digits_of_base(x, 2);

    if !utils::is_palindrome(digits) {
        return false;
    }

    true
}

/// Compute the sum of all double-base palindrome under a given limit.
fn sum_double_base_palindromes(limit: usize) -> usize {
    let mut sum = 0;

    for n in 1..limit {
        if is_double_base_palindrome(n) {
            sum += n;
        }
    }
    sum
}

/// Solve the problem #36 and return the solution.
pub fn solve() -> String {
    sum_double_base_palindromes(1000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(sum_double_base_palindromes(15), 25);
    }

    #[test]
    fn test_is_double_base_palindrome_true() {
        assert!(is_double_base_palindrome(585));
    }

    #[test]
    fn test_is_double_base_palindrome_false() {
        assert!(!is_double_base_palindrome(19));
    }
}
