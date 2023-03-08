use crate::utils;

/// Find the size of the spiral's side which give a ratio of prime on the
/// diagonals under the given ratio.
fn spiral_len_prime_ratio_under(max_ratio: f64) -> usize {
    let mut spiral_size = 0;
    let mut x = 1;
    let mut ratio = 1.0;
    let mut n_prime = 0.0;
    let mut n = 1.0;

    while ratio > max_ratio {
        spiral_size += 2;
        for _ in 0..4 {
            x += spiral_size;
            if utils::is_prime(x) {
                n_prime += 1.0;
            }
            n += 1.0;
        }
        ratio = n_prime / n;
    }
    spiral_size + 1
}

/// Solve the problem #58 and return the solution.
pub fn solve() -> String {
    spiral_len_prime_ratio_under(0.1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        assert_eq!(spiral_len_prime_ratio_under(0.53), 9);
    }
}
