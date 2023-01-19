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
        let mut permutations: Vec<usize> = utils::permutations_of(utils::digits_of(p)).into_iter().map(|x| utils::digits_to_number(x)).filter(|x| utils::is_prime(*x)).collect();

        // Ignore the existing sequence
        if permutations.contains(&1487) {
            continue;
        }

        // If we passed all the constraints, we found the other sequence !
        if permutations.len() == 3 {
            // Sort them and concatenate them
            permutations.sort();

            let mut result = String::new();
            for n in permutations.iter() {
                result.push_str(&n.to_string());
            }
            return result;
        }
    }
    panic!("Coudln't find the other sequence ?!")
}

/// Solve the problem #49 and return the solution.
pub fn solve() -> String {
    prime_permutation()
}
