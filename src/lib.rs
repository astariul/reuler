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
//! ## Example of command line usage
//! You can also call the command line `reuler` directly to get the result.
//!
//! ```console
//! reuler 1
//! ```
//!
//! Will print :
//! > Solution for problem #1 : 233168  
//! > Time taken : 83.208µs

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
        9 => return Ok(problems::prob_9::solve()),
        10 => return Ok(problems::prob_10::solve()),
        11 => return Ok(problems::prob_11::solve()),
        12 => return Ok(problems::prob_12::solve()),
        13 => return Ok(problems::prob_13::solve()),
        14 => return Ok(problems::prob_14::solve()),
        15 => return Ok(problems::prob_15::solve()),
        16 => return Ok(problems::prob_16::solve()),
        17 => return Ok(problems::prob_17::solve()),
        18 => return Ok(problems::prob_18::solve()),
        19 => return Ok(problems::prob_19::solve()),
        20 => return Ok(problems::prob_20::solve()),
        21 => return Ok(problems::prob_21::solve()),
        22 => return Ok(problems::prob_22::solve()),
        23 => return Ok(problems::prob_23::solve()),
        24 => return Ok(problems::prob_24::solve()),
        25 => return Ok(problems::prob_25::solve()),
        26 => return Ok(problems::prob_26::solve()),
        27 => return Ok(problems::prob_27::solve()),
        28 => return Ok(problems::prob_28::solve()),
        29 => return Ok(problems::prob_29::solve()),
        30 => return Ok(problems::prob_30::solve()),
        31 => return Ok(problems::prob_31::solve()),
        32 => return Ok(problems::prob_32::solve()),
        33 => return Ok(problems::prob_33::solve()),
        34 => return Ok(problems::prob_34::solve()),
        35 => return Ok(problems::prob_35::solve()),
        36 => return Ok(problems::prob_36::solve()),
        37 => return Ok(problems::prob_37::solve()),
        38 => return Ok(problems::prob_38::solve()),
        39 => return Ok(problems::prob_39::solve()),
        40 => return Ok(problems::prob_40::solve()),
        41 => return Ok(problems::prob_41::solve()),
        42 => return Ok(problems::prob_42::solve()),
        43 => return Ok(problems::prob_43::solve()),
        44 => return Ok(problems::prob_44::solve()),
        45 => return Ok(problems::prob_45::solve()),
        46 => return Ok(problems::prob_46::solve()),
        47 => return Ok(problems::prob_47::solve()),
        48 => return Ok(problems::prob_48::solve()),
        49 => return Ok(problems::prob_49::solve()),
        50 => return Ok(problems::prob_50::solve()),
        51 => return Ok(problems::prob_51::solve()),
        52 => return Ok(problems::prob_52::solve()),
        53 => return Ok(problems::prob_53::solve()),
        54 => return Ok(problems::prob_54::solve()),
        55 => return Ok(problems::prob_55::solve()),
        56 => return Ok(problems::prob_56::solve()),
        57 => return Ok(problems::prob_57::solve()),
        58 => return Ok(problems::prob_58::solve()),
        59 => return Ok(problems::prob_59::solve()),
        67 => return Ok(problems::prob_67::solve()),
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
