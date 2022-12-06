use crate::utils;

/// Compute the index of the first Fibonacci term with n digits.
fn n_digit_fibonacci(n: u32) -> usize {
    if n == 0 {
        panic!("A number can't have 0 digits !");
    }
    
    let base = 10_usize.pow(n - 1);
    let fibo = utils::Fibonacci { curr: 1, next: 1 };

    for (i, x) in fibo.enumerate() {
        if x / base > 0 {
            return i + 1;
        }
    }

    panic!("Unreachable code");
}

/// Solve the problem #24 and return the solution.
pub fn solve() -> String {
    n_digit_fibonacci(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(n_digit_fibonacci(3), 12);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        assert_eq!(n_digit_fibonacci(0), 12);
    }

    #[test]
    fn test_base_case() {
        assert_eq!(n_digit_fibonacci(1), 1);
    }
}
