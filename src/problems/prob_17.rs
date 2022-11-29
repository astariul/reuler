/// Convert a number in its words equivalent. The second argument should always
/// be set to `false`, it is used internally for recursive calls.
///
/// # Notes
/// This function can hundle numbers up until 999 999. Numbers above this will
/// returns an Error.
fn write_number_in_words(n: usize, sub: bool) -> Result<String, String> {
    // Return error for inputs that we can't handle
    if n >= 1000000 {
        return Err(String::from(
            "`write_number_in_words()` cannot handle 1 000 000 and above.",
        ));
    }

    Ok(match n {
        // Base cases : write the numbers directly
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        // Other cases : the number is made of several parts
        // Decompose the number, and write each part to build the full number
        _ => {
            if n / 1000 > 0 {
                let thousands = write_number_in_words(n / 1000, true).unwrap();
                match n % 1000 {
                    0 => format!("{} thousand", thousands),
                    r => match r / 100 {
                        0 => format!(
                            "{} thousand and {}",
                            thousands,
                            write_number_in_words(r, false).unwrap()
                        ),
                        _ => format!(
                            "{} thousand {}",
                            thousands,
                            write_number_in_words(r, false).unwrap()
                        ),
                    },
                }
            } else if n / 100 > 0 {
                let hundreds = write_number_in_words(n / 100, true).unwrap();
                match n % 100 {
                    0 => format!("{} hundred", hundreds),
                    r if sub => format!(
                        "{} hundred {}",
                        hundreds,
                        write_number_in_words(r, false).unwrap()
                    ),
                    r => format!(
                        "{} hundred and {}",
                        hundreds,
                        write_number_in_words(r, false).unwrap()
                    ),
                }
            } else {
                let remains = match n % 10 {
                    0 => String::from(""),
                    r => format!("-{}", write_number_in_words(r, false).unwrap()),
                };
                match n / 10 {
                    2 => format!("twenty{}", remains),
                    3 => format!("thirty{}", remains),
                    4 => format!("forty{}", remains),
                    5 => format!("fifty{}", remains),
                    6 => format!("sixty{}", remains),
                    7 => format!("seventy{}", remains),
                    8 => format!("eighty{}", remains),
                    9 => format!("ninety{}", remains),
                    _ => panic!("Unreachable code"),
                }
            }
        }
    })
}

/// Count the number of letters in the numbers written in words (up to a given
/// limit).
fn count_number_letters(limit: usize) -> Result<usize, String> {
    let mut sum = 0;

    for i in 1..limit + 1 {
        let word = write_number_in_words(i, false)?;

        // Remove spaces and hyphens, to count only letters
        sum += word.replace(" ", "").replace("-", "").len();
    }

    Ok(sum)
}

/// Solve the problem #17 and return the solution.
pub fn solve() -> String {
    count_number_letters(1000).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(count_number_letters(5).unwrap(), 19);
    }

    #[test]
    fn test_writer_too_big() {
        assert!(write_number_in_words(1000000, false).is_err());
    }

    #[test]
    fn test_writer_zero() {
        assert_eq!(write_number_in_words(0, false).unwrap(), "zero");
    }

    #[test]
    fn test_writer_full() {
        assert_eq!(
            write_number_in_words(1225, false).unwrap(),
            "one thousand two hundred and twenty-five"
        );
    }

    #[test]
    fn test_writer_thousand_alone() {
        assert_eq!(
            write_number_in_words(3000, false).unwrap(),
            "three thousand"
        );
    }

    #[test]
    fn test_writer_hundred_alone() {
        assert_eq!(write_number_in_words(500, false).unwrap(), "five hundred");
    }

    #[test]
    fn test_writer_dec_alone() {
        assert_eq!(write_number_in_words(40, false).unwrap(), "forty");
    }

    #[test]
    fn test_writer_dec_without_hundred() {
        assert_eq!(
            write_number_in_words(1068, false).unwrap(),
            "one thousand and sixty-eight"
        );
    }

    #[test]
    fn test_writer_ext_call() {
        assert_eq!(
            write_number_in_words(120, false).unwrap(),
            "one hundred and twenty"
        );
    }

    #[test]
    fn test_writer_int_call() {
        assert_eq!(
            write_number_in_words(120, true).unwrap(),
            "one hundred twenty"
        );
    }

    #[test]
    fn test_writer_hundred_thousand() {
        assert_eq!(
            write_number_in_words(240005, false).unwrap(),
            "two hundred forty thousand and five"
        );
    }
}
