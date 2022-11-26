/// Compute the number of routes in a grid of given size, starting from top
/// left corner, going to bottom right corner, with only right and down moves.
fn compute_nb_lattice_paths(grid_size: usize) -> usize {
    // Create the map of paths, that will hold the number of paths for each point in the grid
    let mut paths_grid = vec![vec![0; grid_size + 1]; grid_size + 1];

    // Initialize the starting points (top borders and left borders)
    for i in 0..grid_size + 1 {
        paths_grid[0][i] = 1;
        paths_grid[i][0] = 1;
    }

    // Iterate the grid from left to right, top to bottom
    for i in 1..grid_size + 1 {
        for j in 1..grid_size + 1 {
            paths_grid[i][j] = paths_grid[i - 1][j] + paths_grid[i][j - 1];
        }
    }

    // Get the accumulated number of paths from the bottom right cell
    paths_grid[grid_size][grid_size]
}

/// Solve the problem #15 and return the solution.
pub fn solve() -> String {
    compute_nb_lattice_paths(20).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(compute_nb_lattice_paths(2), 6);
    }
}
