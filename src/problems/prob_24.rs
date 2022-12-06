/// Compute all possible permutations for a given vector of elements.
///
/// # Note
/// This function returns the permutations in lexical order (the given elements
/// are first sorted).
fn permutations(elements: &mut Vec<char>) -> Vec<String> {
    let mut permuts = vec![];

    // We want the lexical order, so sort the given elements
    elements.sort();

    loop {
        // Add the current permutation (turn the list of characters into string before)
        let current_permutation = elements.clone().iter().map(|c| c.to_string()).collect();
        permuts.push(current_permutation);

        // get the next permutation in lexicographical order
        let mut i = elements.len() - 1;
        while i > 0 && elements[i - 1] >= elements[i] {
            i -= 1;
        }

        // If there are no more permutations, break out of the loop
        if i == 0 {
            break;
        }

        let mut j = elements.len() - 1;
        while elements[j] <= elements[i - 1] {
            j -= 1;
        }

        elements.swap(i - 1, j);
        elements[i..].reverse();
    }

    permuts
}


/// Compute the n-th permutation for up to the given digit.
fn n_permutation(digit_limit: usize, n: usize) -> String {
    let mut digits = (0..digit_limit + 1).map(|d| char::from_digit(d as u32, 10).unwrap()).collect();
    let mut permuts = permutations(&mut digits);

    let x = permuts.len();

    // Return the right indice
    permuts.remove(n - 1)
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
        let r = permutations(&mut (0..3).map(|d| char::from_digit(d as u32, 10).unwrap()).collect());
        assert_eq!(r.len(), 6);
        assert!(r.contains(&String::from("012")));
        assert!(r.contains(&String::from("021")));
        assert!(r.contains(&String::from("102")));
        assert!(r.contains(&String::from("120")));
        assert!(r.contains(&String::from("012")));
        assert!(r.contains(&String::from("201")));
        assert!(r.contains(&String::from("210")));
    }
}
