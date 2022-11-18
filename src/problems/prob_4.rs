/// Compute the largest palindrome made of the product of x digits, where x is
/// given.
fn largest_palindrome_product(n_digits: u32) -> usize {
    let max_digit = 10_usize.pow(n_digits);
    let mut largest = 0;
    for i in 1..max_digit {
        for j in 1..max_digit {
            let prod = i * j;
            if is_palindromic(prod) && prod > largest {
                largest = prod;
            }
        }
    }
    largest
}

/// Function checking if the given number is palindromic (if it can read both
/// way).
fn is_palindromic(x: usize) -> bool {
    let x_str = x.to_string();
    let len = x_str.len();
    let mid = len / 2;

    // Depending on the size of the string, consider or not the digit in the middle
    let s1 = &x_str[0..mid];
    let s2;
    if len % 2 == 0 {
        s2 = &x_str[mid..len];
    } else {
        s2 = &x_str[mid + 1..len];
    }

    // Reverse the second string, and compare them
    let s2 = s2.chars().rev().collect::<String>();
    s1 == s2
}

/// Solve the problem #2 and return the solution.
pub fn solve() -> String {
    largest_palindrome_product(3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }

    #[test]
    fn test_is_palindromic_true_odd() {
        assert!(is_palindromic(909));
    }

    #[test]
    fn test_is_palindromic_true_even() {
        assert!(is_palindromic(9009));
    }

    #[test]
    fn test_is_palindromic_false_odd() {
        assert!(!is_palindromic(906));
    }

    #[test]
    fn test_is_palindromic_false_even() {
        assert!(!is_palindromic(9099));
    }
}
