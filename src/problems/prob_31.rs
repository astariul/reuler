/// Compute the number of possible ways to make a specific amount (goal) with
/// the given coins. This function assumes coins are sorted.
fn coin_ways(coins: &Vec<usize>, goal: usize) -> usize {
    // Base case
    if goal == 0 {
        return 1;
    }

    // Recursive case
    let mut n_ways = 0;
    for i in 0..coins.len() {
        if coins[i] <= goal {
            n_ways += coin_ways(&coins[..i + 1].to_vec(), goal - coins[i]);
        }
    }
    n_ways
}


/// Compute the number of possible ways to make a specific amount (goal) with
/// the UK coins : 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
fn coin_ways_uk(goal: usize) -> usize {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    coin_ways(&coins, goal)
}


/// Solve the problem #31 and return the solution.
pub fn solve() -> String {
    coin_ways_uk(200).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let coins = vec![1, 2, 5];
        assert_eq!(coin_ways(&coins, 6), 5);
    }
}
