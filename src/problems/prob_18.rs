use std::cmp;

struct Triangle {
    pub rows: Vec<Vec<usize>>,
}

impl Triangle {
    /// Create a Triangle from a string. The string describing the triangle
    /// should be : one row per line, the first line has a single number, and
    /// each line having one more number than the previous (separated by space)
    pub fn new(t: &str) -> Self {
        let mut triangle: Vec<Vec<usize>> = Vec::new();
        for line in t.lines() {
            let mut row = Vec::new();
            for number in line.split_whitespace() {
                let number: usize = number.parse().unwrap();
                row.push(number);
            }

            // Check that the size match
            if triangle.len() > 0 {
                if row.len() != triangle[triangle.len() - 1].len() + 1 {
                    panic!("The given string does not represent a triangle. Can't construct a Triangle.")
                }
            }

            triangle.push(row);
        }

        Triangle { rows: triangle }
    }

    /// Find the maximum sum path from the top of the triangle to the bottom.
    /// Don't explore all paths, it takes too long. Instead, start from the
    /// bottom, and iteratively remove the path that are not optimal.
    pub fn max_path(&self) -> usize {
        let mut curr_sum_paths = vec![0; self.rows.len() + 1];

        // Start from the bottom
        for row_index in (0..self.rows.len()).rev() {
            let mut curr_row = self.rows[row_index].clone();

            // Update the value by taking the best path from the lower rows
            for (i, elem) in curr_row.iter_mut().enumerate() {
                *elem += cmp::max(curr_sum_paths[i], curr_sum_paths[i + 1]);
            }

            // This is our row of best path for the next row
            curr_sum_paths = curr_row
        }
        curr_sum_paths[0]
    }
}

/// Solve the problem #18 and return the solution.
pub fn solve() -> String {
    let triangle = Triangle::new("75\n95 64\n17 47 82\n18 35 87 10\n20 04 82 47 65\n19 01 23 75 03 34\n88 02 77 73 07 63 67\n99 65 04 28 06 16 70 92\n41 41 26 56 83 40 80 70 33\n41 48 72 33 47 32 37 16 94 29\n53 71 44 65 25 43 91 52 97 51 14\n70 11 33 28 77 73 17 78 39 68 17 57\n91 71 52 38 17 14 91 43 58 50 27 29 48\n63 66 04 68 89 53 67 30 73 16 69 87 40 31\n04 62 98 27 23 09 70 98 73 93 38 53 60 04 23");
    triangle.max_path().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let triangle = Triangle::new("3\n7 4\n2 4 6\n8 5 9 3");
        assert_eq!(triangle.max_path(), 23);
    }
}
