use crate::utils;

/// Compute the largest n-digit pandigital number that is also prime.
fn largest_pandigital_prime() -> usize {
    let mut largest_number = 0;

    for n in 1..10 {
        let mut digits = Vec::new();
        for i in 1..n + 1 {
            digits.push(i);
        }

        for p in utils::permutations_of(digits) {
            // Skip even numbers, we know they are not prime
            if p[p.len() - 1] % 2 == 0 {
                continue;
            }

            // Reconstruct the number from its digit
            let mut number = 0;
            let mut m = 1;
            for d in p.iter() {
                number += d * m;
                m *= 10;
            }

            // Check if it fits what we are looking for
            if utils::is_prime(number) && number > largest_number {
                largest_number = number;
            }
        }
    }
    largest_number
}

/// Solve the problem #41 and return the solution.
pub fn solve() -> String {
    largest_pandigital_prime().to_string()
}
