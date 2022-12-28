use crate::utils;

/// Compute the sum of all truncatable primes.
fn sum_truncatable_primes() -> usize {
    let mut sum = 0;
    let mut n_truncatable_primes = 0;
    let mut curr_primes = vec![2, 3, 5, 7];

    // Iteratively add more digits to the right to build primes with x digits
    'inf: loop {
        let mut next_primes = Vec::new();
        for p in curr_primes.iter() {
            for i in [1, 3, 5, 7, 9] {
                let next_p = p * 10 + i;
                if utils::is_prime(next_p) {
                    next_primes.push(next_p);
                }
            }
        }

        // Because of the way we build our primes, we know they are right-truncatable
        // So just check if they are left-truncatable !
        for p in next_primes.iter() {
            let mut is_left_truncatable = true;
            let mut m = 10;
            while m < *p {
                if !utils::is_prime(p % m) {
                    is_left_truncatable = false;
                    break;
                }
                m *= 10;
            }

            if is_left_truncatable {
                sum += p;
                n_truncatable_primes += 1;
            }

            if n_truncatable_primes == 11 {
                break 'inf;
            }
        }

        curr_primes = next_primes;
    }
    sum
}

/// Solve the problem #37 and return the solution.
pub fn solve() -> String {
    sum_truncatable_primes().to_string()
}
