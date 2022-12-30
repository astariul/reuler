use std::collections::HashSet;

/// Compute the largest pandigital 9-digit number that can be formed as the
/// concatenated product pf an integer with (1, 2, ..., n) where n > 1.
fn largest_pandigital_concatenated_product() -> usize {
    let mut best_pandigital = 0;

    // Because n is at least 2, the initial integer is at most 4 digits
    for i in 0..10000 {
        let mut n = 1;
        let mut concat_product = String::new();
        loop {
            concat_product.push_str(&(i * n).to_string());

            // Count the number of different characters
            let unique_digits = HashSet::<char>::from_iter(concat_product.chars());
            if unique_digits.len() < concat_product.len() || unique_digits.contains(&'0') {
                // Early stop in one of the following case :
                // -> A character is duplicated, it's not pandigital
                // -> It contains 0, so it's not 1-to-9 pandigital
                break;
            } else if concat_product.len() == 9 {
                // It's pandigital !
                let curr_pandigital: usize = concat_product.parse().unwrap();
                if curr_pandigital > best_pandigital {
                    best_pandigital = curr_pandigital;
                }
                break;
            }

            // For next iteration
            n += 1;
        }
    }
    best_pandigital
}

/// Solve the problem #38 and return the solution.
pub fn solve() -> String {
    largest_pandigital_concatenated_product().to_string()
}
