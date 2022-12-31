use std::collections::HashMap;

/// Compute the p (under a given limit) that maximizes the number of right
/// angle triangle such that the perimeter of the triangle is p.
fn which_p_maximize_n_triangle(limit: usize) -> usize {
    // What we can do is simply iterate all possible right-angle triangle and
    // count the number of triangle of each perimeter size.
    let mut counts = HashMap::new();

    // No need to iterate over limit / 2, because in that case the perimeter
    // will be above the limit ! 
    for c in 1..limit / 2 {
        // a and b are under c, because a^2 + b^2 = c^2
        for b in 1..c + 1 {
            for a in 1..c + 1 {
                if a * a + b * b == c * c {
                    // Right-angle triangle ! Add a count for that perimeter
                    let p = a + b + c;
                    if p <= limit {
                        *counts.entry(a + b + c).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    *counts.iter().max_by_key(|entry| entry.1).unwrap_or((&0, &0)).0
}

/// Solve the problem #39 and return the solution.
pub fn solve() -> String {
    which_p_maximize_n_triangle(1000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_case_no_solution() {
        assert_eq!(which_p_maximize_n_triangle(5), 0);
    }

    #[test]
    fn test_given_example() {
        // p = 120 has 3 possible solutions, which is the best p (until 150)
        assert_eq!(which_p_maximize_n_triangle(150), 120);
    }
}
