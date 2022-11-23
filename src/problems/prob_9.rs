/// Compute a pythagorean triplet that sums to the given number, and return the
/// product of the triplet.
fn pythagorean_triplet(result: usize) -> usize {
    if result < 6 {
        panic!("Pythagorean triplets are natural number, so the minimum result is 6 ({result} given) because 1 + 2 + 3 = 6");
    }

    // We start from 1 because we know a, b, c are natural numbers
    // We end at the expected sum result because we know that a < b < c
    for a in 1..result - 2 + 1 {
        for b in a + 1..result - 1 + 1 {
            for c in b + 1..result + 1 {
                // Check if they sum to the expected result first, it's faster
                // than checking if they are a pythagorean triplet
                if a + b + c == result {
                    if a * a + b * b == c * c {
                        return a * b * c;
                    }
                }
            }
        }
    }
    panic!("There is no pythagorean triplet that satisfies this sum ({result})");
}

/// Solve the problem #9 and return the solution.
pub fn solve() -> String {
    pythagorean_triplet(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(pythagorean_triplet(12), 60);
    }

    #[test]
    #[should_panic]
    fn test_too_small_result() {
        pythagorean_triplet(2);
    }

    #[test]
    #[should_panic]
    fn test_non_existing_triplet() {
        pythagorean_triplet(7);
    }
}
