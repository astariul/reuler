use crate::utils;

/// Find the other sequence of 3 4-digits primes that are permutations of each
/// other. The first one being 1487, 4817, 8147.
fn prime_permutation() -> String {
    for p in utils::Primes::new_up_to(10000) {
        // Only consider 4-digits numbers
        if p < 1000 {
            continue;
        }

        // Find all possible permutations, but only keep the primes
        let mut perm: Vec<usize> = utils::permutations_of(utils::digits_of(p))
            .into_iter()
            .map(|x| utils::digits_to_number(x))
            .filter(|x| utils::is_prime(*x))
            .collect();

        // Dedup the permutations (need sorting)
        perm.sort();
        perm.dedup();

        // If the size is right, check that the difference between each element
        // is the same
        if perm.len() >= 3 {
            // We have to check all 3-subsequences
            for i in 0..perm.len() - 3 + 1 {
                if perm[i + 1] - perm[i] == perm[i + 2] - perm[i + 1] {
                    // We passed all the constraints ! Concatenate them and return
                    let mut result = String::new();
                    result.push_str(&perm[i].to_string());
                    result.push_str(&perm[i + 1].to_string());
                    result.push_str(&perm[i + 2].to_string());

                    // Only return if the result is not the existing sequence
                    if result != "148748178147" {
                        return result;
                    }
                }
            }
        }
    }
    panic!("Coudln't find the other sequence ?!")
}

/// Solve the problem #49 and return the solution.
pub fn solve() -> String {
    prime_permutation()
}
