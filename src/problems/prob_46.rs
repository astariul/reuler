/// Find the smallest odd composite number that cannot be written as the sum of
/// a prime and twice a square.
fn goldbach_conjecture_counter_example() -> usize {
    let mut previous_primes = vec![2, 3, 5, 7];
    let mut x = 7;

    loop {
        // Skip even-numbers
        x += 2;

        // Check if the number is divisible by any previous prime
        let mut divisible = false;
        for pp in previous_primes.iter().rev() {
            if x % pp == 0 {
                divisible = true;
                break;
            }
        }

        if !divisible {
            // x is prime, skip it
            previous_primes.push(x);
            continue;
        }

        // From here, x is an odd-composite number
        // Check all previous prime numbers, and see if we can write this
        // as a prime + twice a square
        let mut conjecture_is_right = false;
        for pp in previous_primes.iter().rev() {
            let square = (x - pp) / 2;
            let s = (square as f64).sqrt() as usize;
            if 2 * s * s + pp == x {
                // The conjecture holds...
                conjecture_is_right = true;
                break;
            }
        }

        if !conjecture_is_right {
            // We found a number for which the conjecture is not true !
            return x;
        }
    }
}

/// Solve the problem #46 and return the solution.
pub fn solve() -> String {
    goldbach_conjecture_counter_example().to_string()
}
