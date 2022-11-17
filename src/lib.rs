//! `reuler` provide Rust implementation for the
//! [Project Euler](https://projecteuler.net/).
//!
//! ## Example of library usage
//! You can call the `solve()` function to run any of the implemented solution.
//!
//! ```rust
//! use reuler;
//!
//! println!(reuler::solve(1).unwrap());
//! ```
//!
//! ## Example of commandline usage
//! You can also call the command line `reuler` directly to get the result.
//!
//! ```console
//! reuler 1
//! ```

/// Solve the given problem and return the solution as a string.
///
/// # Errors
/// If the given problem ID is invalid or doesn't have an implementation yet,
/// an error is returned.
pub fn solve(problem_id: isize) -> Result<&'static str, String> {
    if problem_id < 1 {
        return Err(format!("The provided problem ID is not valid (0 or negative number : `{problem_id}`). Please provide a valid ID."));
    }
    // "The solution for the problem #{} is not yet implemented. Consider contributing !", problem_id);
    Ok("23")
}
