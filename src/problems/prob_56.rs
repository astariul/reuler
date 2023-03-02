use crate::utils;

/// Find the maximum sum of a^b, where a and b can be any number under the
/// given limit.
fn max_power_sum(limit: usize) -> usize {
    let mut max_sum = 0;
    for a in 1..limit {
        let mut x = utils::BigInt::from(a);
        for _b in 1..limit {
            x *= a;
            let digit_sum = x.digits.iter().sum();
            if digit_sum > max_sum {
                max_sum = digit_sum
            }
        }
    }
    max_sum
}

/// Solve the problem #56 and return the solution.
pub fn solve() -> String {
    max_power_sum(100).to_string()
}
