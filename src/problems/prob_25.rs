use crate::utils;

/// Compute the index of the first Fibonacci term with n digits.
fn n_digit_fibonacci(n: usize) -> usize {
    if n == 0 {
        panic!("A number can't have 0 digits !");
    }

    let mut curr = utils::BigInt::from(1);
    let mut next = utils::BigInt::from(1);
    let mut i = 1;

    while curr.to_string().len() < n {
        let next_next = &curr + &next;
        curr = next;
        next = next_next;
        
        i += 1;
    }
    i
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
