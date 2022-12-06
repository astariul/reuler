/// Iterator to compute all possible permutations for a given vector of
/// elements.
///
/// # Note
/// This Iterator returns the permutations in lexical order (sorted).
struct Permutations {
    elements: Vec<char>,
    is_end: bool,
}

impl Permutations {
    /// Constructor, making sure the elements are sorted.
    pub fn new(elements: Vec<char>) -> Self {
        let mut elements = elements.clone();

        // We want the lexical order, so sort the given elements
        elements.sort();

        Self {
            elements: elements,
            is_end: false,
        }
    }
}

/// Implement `Iterator` for `Permutations`.
impl Iterator for Permutations {
    type Item = String;

    /// Define how to compute the next step.
    fn next(&mut self) -> Option<Self::Item> {
        // If the last iteration was the last one, is_end is true : return None
        if self.is_end {
            return None;
        }

        // Get the current permutation (turn the list of characters into string before)
        // This is what we will return for this iteration
        let current_permutation = self
            .elements
            .clone()
            .iter()
            .map(|c| c.to_string())
            .collect();

        // From here, compute the next permutation in lexicographical order
        let mut i = self.elements.len() - 1;
        while i > 0 && self.elements[i - 1] >= self.elements[i] {
            i -= 1;
        }

        // If there are no more permutations, this is the last iteration
        // (the next one should return None)
        if i == 0 {
            self.is_end = true;
        } else {
            let mut j = self.elements.len() - 1;
            while self.elements[j] <= self.elements[i - 1] {
                j -= 1;
            }

            self.elements.swap(i - 1, j);
            self.elements[i..].reverse();
        }

        // Return the current permutation !
        Some(current_permutation)
    }
}

/// Compute the n-th permutation for up to the given digit.
fn n_permutation(digit_limit: usize, n: usize) -> String {
    let digits = (0..digit_limit + 1)
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect();
    let permuts = Permutations::new(digits);

    for (i, p) in permuts.enumerate() {
        if i == n - 1 {
            return p;
        }
    }

    panic!(
        "The given n ({n}) is too big, there is not that much permutations. Provide a smaller n."
    )
}

/// Solve the problem #24 and return the solution.
pub fn solve() -> String {
    n_permutation(9, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(n_permutation(2, 1), String::from("012"));
        assert_eq!(n_permutation(2, 2), String::from("021"));
        assert_eq!(n_permutation(2, 3), String::from("102"));
        assert_eq!(n_permutation(2, 4), String::from("120"));
        assert_eq!(n_permutation(2, 5), String::from("201"));
        assert_eq!(n_permutation(2, 6), String::from("210"));
    }

    #[test]
    fn test_permutations() {
        let mut permuts = Permutations::new(
            (0..3)
                .map(|d| char::from_digit(d as u32, 10).unwrap())
                .collect(),
        );
        assert_eq!(permuts.next().unwrap(), String::from("012"));
        assert_eq!(permuts.next().unwrap(), String::from("021"));
        assert_eq!(permuts.next().unwrap(), String::from("102"));
        assert_eq!(permuts.next().unwrap(), String::from("120"));
        assert_eq!(permuts.next().unwrap(), String::from("201"));
        assert_eq!(permuts.next().unwrap(), String::from("210"));
        assert!(permuts.next().is_none());
    }
}
