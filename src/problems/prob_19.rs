/// Function computing if a given year is a leap year or not.
fn is_leap(year: usize) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

/// Return the number of Sundays that are the first day of the month between
/// the given years (included).
///
/// # Note
/// This function does not work for any year prior 1900. It will returns an
/// Error if you try to pass a starting year prior 1900.
fn nb_sundays_first_of_month_between(start_year: usize, end_year: usize) -> Result<usize, String> {
    // Handle wrong inputs
    if start_year < 1900 {
        return Err(String::from(
            "Can't compute for year prior 1900. Please specify `start_year` >= 1900",
        ));
    }

    // We know the 1 january 1900 was a Monday
    let (mut day, mut month, mut year) = (1, 1, 1900);
    let mut i = 0;
    let mut n_sundays = 0;

    let is_sunday = |i| i % 7 == 6; // Monday is 0, so Sunday is 6

    // Iterate day by day
    while year <= end_year {
        // If this is the proper year, it's a Sunday and the first of the month, count !
        if year >= start_year && is_sunday(i) && day == 1 {
            n_sundays += 1;
        }

        // Update the date for the next iteration
        i += 1;

        // First, the days and the months
        match month {
            4 | 6 | 9 | 11 if day == 30 => {
                day = 1;
                month += 1;
            }
            1 | 3 | 5 | 7 | 8 | 10 | 12 if day == 31 => {
                day = 1;
                month += 1;
            }
            2 if !is_leap(year) && day == 28 => {
                day = 1;
                month += 1;
            }
            2 if is_leap(year) && day == 29 => {
                day = 1;
                month += 1;
            }
            _ => {
                day += 1;
            }
        }

        // Update the year if necessary
        if month > 12 {
            month = 1;
            year += 1;
        }
    }
    Ok(n_sundays)
}

/// Solve the problem #19 and return the solution.
pub fn solve() -> String {
    nb_sundays_first_of_month_between(1901, 2000)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(nb_sundays_first_of_month_between(1900, 1905).unwrap(), 11);
    }

    #[test]
    fn test_wrong_input() {
        assert!(nb_sundays_first_of_month_between(1805, 1808).is_err());
    }

    #[test]
    fn test_impossible_interval() {
        assert_eq!(nb_sundays_first_of_month_between(1950, 1940).unwrap(), 0);
    }

    #[test]
    fn test_is_leap_year_yes() {
        assert!(is_leap(1616));
    }

    #[test]
    fn test_is_leap_year_no() {
        assert!(!is_leap(1617));
    }

    #[test]
    fn test_is_leap_year_century_yes() {
        assert!(is_leap(1600));
    }

    #[test]
    fn test_is_leap_year_century_no() {
        assert!(!is_leap(1700));
    }
}
