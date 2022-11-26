use std::collections::HashMap;

/// Define a Collatz structure, that will keep intermediate results in memory,
/// using dynamic programming.
struct Collatz {
    mem: HashMap<usize, usize>,
}

impl Collatz {
    /// Create a new Collatz structure.
    pub fn new() -> Self {
        // We already know the sequence lenght for 1, which is the end of every sequence
        let mut init_mem = HashMap::new();
        init_mem.insert(1, 1);

        Collatz { mem: init_mem }
    }

    /// Compute the length of the Collatz sequence from the given number.
    pub fn seq_len(&mut self, n: usize) -> usize {
        // Check if the value is already in memory
        match self.mem.get(&n).copied() {
            Some(seq_len) => seq_len,
            None => {
                // Otherwise compute it recursively
                let seq_len = match n % 2 {
                    0 => self.seq_len(n / 2) + 1,
                    _ => self.seq_len(3 * n + 1) + 1,
                };

                // Update the cache
                self.mem.insert(n, seq_len);

                seq_len
            }
        }
    }
}

/// Compute the starting number under the given limit that produces the longest
/// Collatz sequence.
fn longest_collatz_sequence(limit: usize) -> usize {
    let mut curr_best_start = 1;
    let mut curr_len = 1;

    // Give more memory than the limit, because the numbers in the chain can go over the limit
    let mut collatz = Collatz::new();

    for i in 1..limit {
        let seq_len = collatz.seq_len(i);

        if seq_len > curr_len {
            curr_best_start = i;
            curr_len = seq_len;
        }
    }
    curr_best_start
}

/// Solve the problem #14 and return the solution.
pub fn solve() -> String {
    longest_collatz_sequence(1000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(longest_collatz_sequence(10), 9);
    }
}
