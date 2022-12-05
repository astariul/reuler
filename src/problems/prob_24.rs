/// Compute all possible permutations for a given vector of elements.
fn permutations(elements: Vec<char>) -> Vec<String> {
    let mut elements = elements.clone();

    if elements.len() == 1 {
        return elements.iter().map(|c| c.to_string()).collect();
    }

    // Recursive call
    let extracted_elem = elements.pop().unwrap();
    let size = elements.len();
    let sub_permutations = permutations(elements);

    let mut perms = Vec::new();
    for pos in 0..size + 1 {
        let mut subp = sub_permutations.clone();
        for i in 0..subp.len() {
            subp[i].insert(pos, extracted_elem);
        }
        perms.extend(subp);
    }
    perms
}


/// Compute the n-th permutation for up to the given digit.
fn n_permutation(digit_limit: usize, n: usize) -> String {
    let digits = (0..digit_limit + 1).map(|d| char::from_digit(d as u32, 10).unwrap()).collect();
    let mut permuts = permutations(digits);

    // Sort the permutations alphabetically
    // permuts.sort();

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
        let r = permutations((0..3).map(|d| char::from_digit(d as u32, 10).unwrap()).collect());
        for x in r.iter() {
            println!("{x}");
        }
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
