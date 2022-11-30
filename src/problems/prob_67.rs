use crate::problems::prob_18;


/// Solve the problem #67 and return the solution.
pub fn solve() -> String {
    let triangle_content = include_str!("data/triangle.txt");
    let triangle = prob_18::Triangle::new(triangle_content);
    triangle.max_path().to_string()
}
