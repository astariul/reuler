use std::collections::HashSet;

/// Check if a product of a * b is pandigital or not.
fn is_pandigital_product(a: usize, b: usize) -> bool {
    let ab = a * b;
    let s = format!("{a}{b}{ab}");

    // If we don't have the right size, early return
    if s.len() != 9 {
        return false;
    }

    // If have the right size, ensure we have the all the digits
    let mut digits = HashSet::new();
    for c in s.chars() {
        digits.insert(c.to_string());
    }

    digits.len() == 9 && !digits.contains("0")
}

/// Compute the sum of all products whose multiplicand/multiplier/product
/// identity can be written as a 1 through 9 pandigital.
fn sum_pandigital_products() -> usize {
    let mut pandigital_products = HashSet::new();

    // Find all pandigital products
    // Note that we don't need to iterate every number, we can reduce the
    // iterations. We need a * b = ab, with len(a) + len(b) + len(ab) = 9
    // So we have either :
    // * Case #1 : len(a) = 1, len(b) = 4, len(ab) = 4
    // * Case #2 : len(a) = 2, len(b) = 3, len(ab) = 4
    // And that's it ! (Because a and b can be swapped, so no need to check
    // len(a) = 3 and len(b) = 2, since we already dealed with the equivalent)

    // Case #1
    for a in 1..10 {
        for b in 1001..10000 {
            if is_pandigital_product(a, b) {
                pandigital_products.insert(a * b);
            }
        }
    }

    // Case #2
    for a in 11..100 {
        for b in 101..1000 {
            if is_pandigital_product(a, b) {
                pandigital_products.insert(a * b);
            }
        }
    }

    pandigital_products.iter().sum()
}

/// Solve the problem #32 and return the solution.
pub fn solve() -> String {
    sum_pandigital_products().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pandigital_product_true() {
        assert!(is_pandigital_product(39, 186));
    }

    #[test]
    fn test_is_pandigital_product_false() {
        assert!(!is_pandigital_product(37, 186));
    }

    #[test]
    fn test_is_pandigital_product_wrong_size() {
        assert!(!is_pandigital_product(47, 3096));
    }

    #[test]
    fn test_is_pandigital_product_with_zero() {
        assert!(!is_pandigital_product(3, 1809));
    }
}
