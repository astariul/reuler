use std::collections::HashMap;

fn cached_factorial(n: u128, cache: &mut HashMap<u128, u128>) -> u128 {
    cache.entry(0).or_insert(1);
    if !cache.contains_key(&n) {
        let f = n * cached_factorial(n - 1, cache);
        cache.insert(n, f);
    }
    cache[&n]
}

/// Find the number of combinatorics selections where (n r) > above_x with
/// any r, and 1 <= n <= max_n
fn combinatorics_selections_above(max_n: u128, above_x: u128) -> usize {
    let mut cache = HashMap::new();

    let mut n_exceeding_value = 0;
    for n in 1..max_n + 1 {
        for r in 1..n + 1 {
            println!("Yo wtf");
            let x = cached_factorial(n, &mut cache) / (cached_factorial(r, &mut cache) * cached_factorial(n - r, &mut cache));
            println!("{r} among {n} = {x}");
            if x > above_x {
                n_exceeding_value += 1;
            }
        }
    }
    n_exceeding_value
}

/// Solve the problem #53 and return the solution.
pub fn solve() -> String {
    combinatorics_selections_above(100, 1000000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(combinatorics_selections_above(23, 1000000), 4);
    }
}
