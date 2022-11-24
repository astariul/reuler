struct Grid {
    pub rows: Vec<Vec<usize>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(g: &str) -> Self {
        let mut grid: Vec<Vec<usize>> = Vec::new();
        for line in g.lines() {
            let mut row = Vec::new();
            for number in line.split_whitespace() {
                let number: usize = number.parse().unwrap();
                row.push(number);
            }

            // Check that the size match
            if grid.len() > 0 {
                if row.len() != grid[grid.len() - 1].len() {
                    panic!("The given string does not have a consistent row size. Can't construct a Grid.")
                }
            }

            grid.push(row);
        }

        let h = grid.len();
        let w = grid[h - 1].len();
        Grid {
            rows: grid,
            width: w,
            height: h,
        }
    }
}

/// Compute the largest product of n digits from the given grid, where we
/// consider n adjacent numbers (horizontally, vertically, diagonally)
fn largest_product_in(grid: Grid, n: usize) -> usize {
    let mut max_product = 0;

    // Search vertically
    for i in 0..grid.width {
        for j in 0..grid.height - (n - 1) {
            let mut product = 1;
            for k in 0..n {
                product *= grid.rows[j + k][i];
            }

            if product > max_product {
                max_product = product;
            }
        }
    }

    // Search horizontally
    for i in 0..grid.width - (n - 1) {
        for j in 0..grid.height {
            let mut product = 1;
            for k in 0..n {
                product *= grid.rows[j][i + k];
            }

            if product > max_product {
                max_product = product;
            }
        }
    }

    // Search diagonally (left) : \
    for i in 0..grid.width - (n - 1) {
        for j in 0..grid.height - (n - 1) {
            let mut product = 1;
            for k in 0..n {
                product *= grid.rows[j + k][i + k];
            }

            if product > max_product {
                max_product = product;
            }
        }
    }

    // Search diagonally (right) : /
    for i in n - 1..grid.width {
        for j in 0..grid.height - (n - 1) {
            let mut product = 1;
            for k in 0..n {
                product *= grid.rows[j + k][i - k];
            }

            if product > max_product {
                max_product = product;
            }
        }
    }

    max_product
}

/// Solve the problem #11 and return the solution.
pub fn solve() -> String {
    let grid = Grid::new("08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08\n49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00\n81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65\n52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91\n22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80\n24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50\n32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70\n67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21\n24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72\n21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95\n78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92\n16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57\n86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58\n19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40\n04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66\n88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69\n04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36\n20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16\n20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54\n01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48");
    largest_product_in(grid, 4).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal() {
        let grid = Grid::new("1 2\n3 4");
        assert_eq!(largest_product_in(grid, 2), 12);
    }

    #[test]
    fn test_vertical() {
        let grid = Grid::new("1 3\n2 4");
        assert_eq!(largest_product_in(grid, 2), 12);
    }

    #[test]
    fn test_diagonal_left() {
        let grid = Grid::new("3 1\n2 4");
        assert_eq!(largest_product_in(grid, 2), 12);
    }

    #[test]
    fn test_diagonal_right() {
        let grid = Grid::new("1 3\n4 2");
        assert_eq!(largest_product_in(grid, 2), 12);
    }

    #[test]
    fn test_tall_grid() {
        let grid = Grid::new("64 64\n2 2\n3 2");
        assert_eq!(largest_product_in(grid, 3), 384);
    }

    #[test]
    #[should_panic]
    fn test_inconsistent_grid() {
        Grid::new("1 2\n34");
    }
}
