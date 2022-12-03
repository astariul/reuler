/// Compute the total score for the names given.
fn name_score(names: &str) -> usize {
    // Parse the string
    let names: Vec<&str> = names.split(",").collect();

    // Remove the "" around names
    let mut names: Vec<&str> = names.iter().map(|name| &name[1..name.len() - 1]).collect();

    // Sort alphabetically
    names.sort();

    // Compute the score of each name
    let name_scores: Vec<usize> = names.iter().map(|name| -> usize {
        name.as_bytes()
            .iter()
            .map(|b| usize::from(b - 64))
            .collect::<Vec<usize>>()
            .iter()
            .sum()
    }).collect();
    
    // Compute total score
    name_scores.iter().enumerate().map(|(i, score)| (i + 1) * score).sum()
}

/// Solve the problem #22 and return the solution.
pub fn solve() -> String {
    let names = include_str!("data/names.txt");
    name_score(names).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(name_score("\"COLIN\""), 53);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(name_score("\"COLIN\",\"A\""), 1 * 1 + 53 * 2);
    }
}
