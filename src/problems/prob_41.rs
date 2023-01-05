use crate::utils;

/// Compute the largest n-digit pandigital number that is also prime.
fn largest_pandigital_prime() -> usize {
    // We know from the example that we need at least a 4-pandigital number,
    // so we can skip the cases where n = 1, 2, 3

    // A number is divisible by 3 if the sum of its digits is divisible by 3
    // 1+2+3+4           = 10    -> Not divisible by 3
    // 1+2+3+4+5         = 15    -> Divisible by 3
    // 1+2+3+4+5+6       = 21    -> Divisible by 3
    // 1+2+3+4+5+6+7     = 28    -> Not divisible by 3
    // 1+2+3+4+5+6+7+8   = 36    -> Divisible by 3
    // 1+2+3+4+5+6+7+8+9 = 45    -> Divisible by 3
    // So we can just check 4-pandigital and 7-pandigital, other are not prime

    let mut largest_number = 0;

    // First, check the 7-pandigital
    for p in utils::permutations_of(vec![1, 2, 3, 4, 5, 6, 7]) {
        // Skip even numbers, we know they are not prime
        if p[p.len() - 1] % 2 == 0 {
            continue;
        }

        // Reconstruct the number from its digit
        let number = utils::digits_to_number(p);

        // Check if it fits what we are looking for
        if utils::is_prime(number) && number > largest_number {
            largest_number = number;
        }
    }

    // If we found a 7-pandigital prime, no need to check 4-pandigital !
    if largest_number > 0 {
        return largest_number;
    }

    // Otherwise, check the 4-pandigital as well
    for p in utils::permutations_of(vec![1, 2, 3, 4]) {
        // Skip even numbers, we know they are not prime
        if p[p.len() - 1] % 2 == 0 {
            continue;
        }

        // Reconstruct the number from its digit
        let number = utils::digits_to_number(p);

        // Check if it fits what we are looking for
        if utils::is_prime(number) && number > largest_number {
            largest_number = number;
        }
    }
    largest_number
}

/// Solve the problem #41 and return the solution.
pub fn solve() -> String {
    largest_pandigital_prime().to_string()
}
