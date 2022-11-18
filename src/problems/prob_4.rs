/// Compute the largest palindrome made of the product of x digits, where x is
/// given.
fn largest_palindrome_product(n_digits: usize) -> usize {
    let mut largest = 0;
    for i in 1..10.pow(n_digits) {
        for j in 1..10.pow(n_digits) {
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
///
/// # Example
/// ```
/// assert!(is_palindromic(909))
/// assert!(!is_palindromic(9099))
/// assert!(is_palindromic(9009))
/// ```
fn is_palindromic(x: usize) -> bool {
    let x_str = x.to_string();
    let len = x_str.len();
    let mid = len / 2;

    if len % 2 == 0 {
        return &x_str[0..mid] == &x_str[mid..len]
    } else {
        return &x_str[0..mid] == &x_str[mid + 1..len]
    }
}

/// Solve the problem #2 and return the solution.
pub fn solve() -> String {
    largest_palindrome_product(3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }
}
