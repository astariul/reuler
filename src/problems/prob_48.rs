// Compute x^n mod m, using the algorithm Exponentiation by squaring.
fn mod_pow_by_squaring(x: u128, n: u128, m: u128) -> u128 {
    match n {
        0 => 1,
        n if n % 2 == 0 => mod_pow_by_squaring((x * x) % m, n / 2, m) % m,
        _ => (x * mod_pow_by_squaring((x * x) % m, (n - 1) / 2, m)) % m,
    }
}

/// Find the last ten digits of the serie 1^1 + 2^2 + 3^3 + ... + n^n.
fn last_digits_of_self_powers_till(n: u128) -> usize {
    let mut sum = 0;
    for x in 1..n + 1 {
        sum += mod_pow_by_squaring(x, x, 10000000000);
    }
    (sum % 10000000000).try_into().unwrap()
}

/// Solve the problem #48 and return the solution.
pub fn solve() -> String {
    last_digits_of_self_powers_till(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(last_digits_of_self_powers_till(10), 405071317);
    }

    #[test]
    fn test_mod_pow_base_case() {
        assert_eq!(mod_pow_by_squaring(364, 0, 10), 1);
    }

    #[test]
    fn test_mod_pow_without_mod() {
        assert_eq!(mod_pow_by_squaring(3, 3, 1000), 27);
    }

    #[test]
    fn test_mod_pow_with_mod() {
        assert_eq!(mod_pow_by_squaring(3, 3, 10), 7);
    }

    #[test]
    fn test_mod_pow_even() {
        assert_eq!(mod_pow_by_squaring(4, 4, 10000), 4 * 4 * 4 * 4);
    }

    #[test]
    fn test_mod_pow_big() {
        assert_eq!(mod_pow_by_squaring(25, 25, 10000000000), 3447265625);
    }
}
