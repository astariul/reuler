use crate::utils;

/// Compute the sum of all 0-9 pandigital numbers such that :
/// d2d3d4 is divisible by 2
/// d3d4d5 is divisible by 3
/// d4d5d6 is divisible by 5
/// d5d6d7 is divisible by 7
/// d6d7d8 is divisible by 11
/// d7d8d9 is divisible by 13
/// d8d9d10 is divisible by 17
fn divisible_pandigital() -> usize {
    let mut sum = 0;
    for p in utils::permutations_of(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]) {
        // Consider the number is in reverse order
        // Check all conditions
        if utils::digits_to_number(p[0..3].to_vec()) % 17 == 0 && utils::digits_to_number(p[1..4].to_vec()) % 13 == 0 && utils::digits_to_number(p[2..5].to_vec()) % 11 == 0 && utils::digits_to_number(p[3..6].to_vec()) % 7 == 0 && utils::digits_to_number(p[4..7].to_vec()) % 5 == 0 && utils::digits_to_number(p[5..8].to_vec()) % 3 == 0 && utils::digits_to_number(p[6..9].to_vec()) % 2 == 0 {
            sum += utils::digits_to_number(p);
        }
    }
    sum
}

/// Solve the problem #43 and return the solution.
pub fn solve() -> String {
    divisible_pandigital().to_string()
}
