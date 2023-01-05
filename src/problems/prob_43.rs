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
        let p = utils::digits_to_number(p);
        
        // Check all conditions
        if (p % 1000) % 17 == 0 && (p / 10 % 1000) % 13 == 0 && (p / 100 % 1000) % 11 == 0 && (p / 1000 % 1000) % 7 == 0 && (p / 10000 % 1000) % 5 == 0 && (p / 100000 % 1000) % 3 == 0 && (p / 1000000 % 1000) % 2 == 0 {
            sum += p;
        }
    }
    sum
}

/// Solve the problem #43 and return the solution.
pub fn solve() -> String {
    divisible_pandigital().to_string()
}
