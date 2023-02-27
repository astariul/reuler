use crate::utils;

const ASSUME_LYCHREL_AFTER: usize = 50;

/// Determine if a given number is a Lychrel number of not.
fn is_lychrel(x: usize) -> bool {
    let mut i = 0;
    let mut curr_x = utils::BigInt::from(x);
    loop {
        curr_x = &curr_x + &curr_x.reverse();
        i += 1;

        if curr_x.is_palindrome() {
            return false;
        }

        if i > ASSUME_LYCHREL_AFTER {
            break;
        }
    }
    true
}

/// Find the number of Lychrel numbers under a given limit.
/// Note that we assume a number is a Lychrel number if if can't reach a
/// palindromic number under 50 iterations.
fn lychrel_numbers_under(limit: usize) -> usize {
    let mut n_lychrel = 0;
    for i in 1..limit + 1 {
        if is_lychrel(i) {
            n_lychrel += 1;
        }
    }
    n_lychrel
}

/// Solve the problem #55 and return the solution.
pub fn solve() -> String {
    lychrel_numbers_under(10000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(lychrel_numbers_under(200), 1);
    }
}
