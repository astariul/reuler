use crate::utils;

/// If we write positive integers next to each other, we can count the x-th
/// digit. This function returns the product of the digits #1, #10, #100,
/// etc... with the order as input.
fn champernowne_constant(order: usize) -> usize {
    let mut prod_result = 1;
    let mut integer = 1;
    let mut n = 1;
    let mut d_idx = 10;
    let mut digit_size = 1;
    let mut digit_limit = 10;

    while utils::digits_of(d_idx).len() - 1 <= order {
        integer += 1;
        if integer == digit_limit {
            digit_limit *= 10;
            digit_size += 1;
        }

        if n < d_idx && n + digit_size >= d_idx {
            // If adding the current integer makes us go over the index we are
            // looking for, find the right digit
            let digits = utils::digits_of(integer);
            let i = d_idx - n - 1;  // Get the right index
            prod_result *= digits[digits.len() - i - 1];    // Don't forget the digits are in reverse order
            
            // Don't forget to update the next digit size we are looking for
            d_idx *= 10;
        }
        
        // Just update n to know where we are
        n += digit_size;
    }
    prod_result
}

/// Solve the problem #40 and return the solution.
pub fn solve() -> String {
    champernowne_constant(6).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_order() {
        assert_eq!(champernowne_constant(0), 1);
    }

    #[test]
    fn test_small_order() {
        assert_eq!(champernowne_constant(2), 5);
    }
}
