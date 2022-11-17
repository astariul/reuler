//! `rustere` stands for Rust Template Repository. It's a Github template
//! repository, which is easy to duplicate and useful to get started, with a
//! fully featured repository.
//! Check it out there : [astariul/rustere](https://github.com/astariul/rustere)
//!
//! This crate doesn't contain any useful code, just some dummy code for the
//! sake of the crate.

/// Checks if a number is odd or not. Returns `True` if the number is odd (1,
/// 3, 5, ...) and `False` if the number is even (0, 2, 4, ...).
///
/// # Examples
/// ```
/// assert!(rustere::is_odd(79));
/// ```
///
/// ```
/// assert!(!rustere::is_odd(88));
/// ```
///
/// # Notes
/// This is just a dummy function, to have _some_ code inside this template
/// repository. Nothing else.
pub fn is_odd(x: usize) -> bool {
    x % 2 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd() {
        assert!(is_odd(1));
    }

    #[test]
    fn test_even() {
        assert!(!is_odd(2));
    }

    #[test]
    fn test_zero() {
        assert!(!is_odd(0));
    }
}
