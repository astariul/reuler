//! `reuler` provide Rust implementation for the
//! [Project Euler](https://projecteuler.net/).
//!
//! ## Example of library usage
//! You can call the `solve()` function to run any of the implemented solution.
//!
//! ```rust
//! use reuler;
//!
//! let res = reuler::solve(1).unwrap();
//! println!("Solution : {res}");
//! ```
//!
//! ## Example of commandline usage
//! You can also call the command line `reuler` directly to get the result.
//!
//! ```console
//! reuler 1
//! ```

pub mod problems;
pub mod utils;

/// Solve the given problem and return the solution as a string.
///
/// # Errors
/// If the given problem ID is invalid or doesn't have an implementation yet,
/// an error is returned.
pub fn solve(problem_id: isize) -> Result<String, String> {
    if problem_id < 1 {
        return Err(format!("The provided problem ID is not valid (0 or negative number : `{problem_id}`). Please provide a valid ID."));
    }

    match problem_id {
        1 => return Ok(problems::prob_1::solve()),
        2 => return Ok(problems::prob_2::solve()),
        3 => return Ok(problems::prob_3::solve()),
        4 => return Ok(problems::prob_4::solve()),
        5 => return Ok(problems::prob_5::solve()),
        6 => return Ok(problems::prob_6::solve()),
        7 => return Ok(problems::prob_7::solve()),
        8 => return Ok(problems::prob_8::solve()),
        _ => return Err(format!("The solution for the problem #{problem_id} is not yet implemented. Consider contributing !")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_problem_1() {
        assert_eq!(solve(1).unwrap(), "233168");
    }

    #[test]
    fn test_invalid_id() {
        match solve(-1) {
            Ok(_val) => assert!(
                false,
                "No error raised, even though the given ID was invalid."
            ),
            Err(e) => assert!(e.contains("not valid"), "Wrong error message"),
        }
    }

    #[test]
    fn test_not_implemented() {
        match solve(9999999) {
            Ok(_val) => assert!(
                false,
                "No error raised, even though the given ID was invalid."
            ),
            Err(e) => assert!(e.contains("not yet implemented"), "Wrong error message"),
        }
    }
}
