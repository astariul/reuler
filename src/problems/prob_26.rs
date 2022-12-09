use std::collections::HashSet;

/// Compute the size of the decimal cycle of the fraction 1/n.
fn cycle_size(n: usize) -> usize {
    let mut remainder = 1;
    let mut seen = HashSet::new();
    let mut i = 0;

    loop {
        remainder = (remainder * 10) % n;

        // If we already seen this remainder, it's a cycle
        if seen.contains(&remainder) {
            break;
        }

        // If the remainder is 0, we are not even in  a cycle
        if remainder == 0 {
            return 0;
        }

        // Iteration update
        i += 1;
        seen.insert(remainder);
    }
    i
}

/// Compute the n (<= max) for which 1/n has the biggest cycle size.
fn max_cycle_size_until(max: usize) -> usize {
    let mut cycle_max_size = 0;
    let mut cycle_max_n = 0;

    // We know for any number n, the cycle size can't exceed n
    // So start from the maximum number, and as soon as n is smaller than the
    // current maximum cycle size, we can leave the loop
    for n in (1..max + 1).rev() {
        if n < cycle_max_size {
            // Early break out of the loop
            break;
        }

        let curr_cycle_size = cycle_size(n);

        if curr_cycle_size > cycle_max_size {
            cycle_max_n = n;
            cycle_max_size = curr_cycle_size;
        }
    }
    cycle_max_n
}

/// Solve the problem #26 and return the solution.
pub fn solve() -> String {
    max_cycle_size_until(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(max_cycle_size_until(10), 7);
    }

    #[test]
    fn test_cycle_size() {
        assert_eq!(cycle_size(1), 0);
        assert_eq!(cycle_size(2), 0);
        assert_eq!(cycle_size(3), 1);
        assert_eq!(cycle_size(4), 0);
        assert_eq!(cycle_size(5), 0);
        assert_eq!(cycle_size(6), 1);
        assert_eq!(cycle_size(7), 6);
        assert_eq!(cycle_size(8), 0);
        assert_eq!(cycle_size(9), 1);
        assert_eq!(cycle_size(10), 0);
    }
}
