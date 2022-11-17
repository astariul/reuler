use crate::utils;

/// Compute the sum of even values of the Fibonacci sequence below the given
/// limit.
fn sum_even_fibo_under(limit: usize) -> usize {
    let mut fibo = utils::Fibonacci { curr: 1, next: 2 };
    let mut sum = 0;

    loop {
        let c = fibo.next().unwrap();

        if c > limit {
            break;
        }

        if c % 2 == 0 {
            sum += c;
        }
    }
    sum
}

/// Solve the problem #2 and return the solution.
pub fn solve() -> String {
    sum_even_fibo_under(4000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(sum_even_fibo_under(100), 44);
    }
}
