use crate::utils;

/// Check if a fraction is a non-trivial digit canceling fraction. This
/// function assumes that both the numerator and the denominator are 2 digits.
fn is_non_trivial_digit_canceling_fraction(n: usize, d: usize) -> bool {
    let mut n1 = n % 10;
    let mut n2 = n / 10;
    let mut d1 = d % 10;
    let mut d2 = d / 10;

    // Make it so n1 == d1
    if n1 == d2 {
        (d1, d2) = (d2, d1);
    } else if n2 == d1 {
        (n1, n2) = (n2, n1);
    } else if n2 == d2 {
        (d1, d2) = (d2, d1);
        (n1, n2) = (n2, n1);
    }

    // If nothing match or if it's trivial, it's not a digit cancelling fraction
    if n1 != d1 || n1 == 0 {
        return false;
    }

    // Check if n2 / d2 (the simplified fraction) is equal to the original fraction
    let gcf = utils::gcf(n, d);
    let simple_n = n / gcf;
    let simple_d = d / gcf;

    let gcf = utils::gcf(n2, d2);
    let simple_n2 = n2 / gcf;
    let simple_d2 = d2 / gcf;

    simple_n == simple_n2 && simple_d == simple_d2
}

/// Compute the denominator (simplified) of the product of the four non-trivial
/// digit cancelling fraction.
fn denominator_of_product_digit_cancelling_fractions() -> usize {
    // First, let's find the non-trivial digit cancelling fractions and
    // multiply their numerators / denominators
    let mut prod_num = 1;
    let mut prod_denom = 1;

    for numerator in 10..100 {
        for denominator in numerator + 1..101 {
            if is_non_trivial_digit_canceling_fraction(numerator, denominator) {
                prod_num *= numerator;
                prod_denom *= denominator;
            }
        }
    }

    let gcf = utils::gcf(prod_num, prod_denom);
    prod_denom / gcf
}

/// Solve the problem #33 and return the solution.
pub fn solve() -> String {
    denominator_of_product_digit_cancelling_fractions().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_non_trivial_digit_canceling_fraction_true() {
        assert!(is_non_trivial_digit_canceling_fraction(49, 98));
    }

    #[test]
    fn test_is_non_trivial_digit_canceling_fraction_false() {
        assert!(!is_non_trivial_digit_canceling_fraction(50, 98));
    }

    #[test]
    fn test_is_non_trivial_digit_canceling_fraction_trivial() {
        assert!(!is_non_trivial_digit_canceling_fraction(30, 50));
    }
}
