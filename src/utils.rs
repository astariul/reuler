use std::collections::HashSet;
use std::ops;

/// Iteratively compute the Fibonacci sequence.
///
/// # Examples
/// ```
/// let mut fibo = reuler::utils::Fibonacci { curr: 1, next: 2 };
///
/// assert_eq!(fibo.next().unwrap(), 1);
/// assert_eq!(fibo.next().unwrap(), 2);
/// assert_eq!(fibo.next().unwrap(), 3);
/// assert_eq!(fibo.next().unwrap(), 5);
/// assert_eq!(fibo.next().unwrap(), 8);
/// ```
///
/// Or you can use it in a loop (will never end) :
/// ```
/// let mut fibo = reuler::utils::Fibonacci { curr: 1, next: 2 };
///
/// for f in fibo {
///     // Do something with f, the current fibonacci number
///     
///     // Then at some point, break out of the loop
///     break;
/// }
/// ```
pub struct Fibonacci {
    pub curr: usize,
    pub next: usize,
}

/// Implement `Iterator` for `Fibonacci`.
impl Iterator for Fibonacci {
    type Item = usize;

    /// Define how to compute the next step.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

/// Iteratively compute the primes numbers.
///
/// # Examples
/// Use `new()` for indefinitely iterating the prime numbers.
/// ```
/// let mut primes = reuler::utils::Primes::new();
///
/// assert_eq!(primes.next().unwrap(), 2);
/// assert_eq!(primes.next().unwrap(), 3);
/// assert_eq!(primes.next().unwrap(), 5);
/// assert_eq!(primes.next().unwrap(), 7);
/// assert_eq!(primes.next().unwrap(), 11);
/// ```
///
/// Or you can use `new_up_to()` for all the primes up until the given limit (
/// giving a limit will results in faster computations) :
/// ```
/// let mut primes = reuler::utils::Primes::new_up_to(20);
///
/// let mut last_p = 0;
/// for p in primes {
///     last_p = p;
/// }
/// assert_eq!(last_p, 19);
/// ```
pub struct Primes {
    p: usize,
    previous_primes: Vec<usize>,
    limit: Option<usize>,
}

impl Primes {
    /// Create a new iterator, producing indefinitely prime numbers.
    /// Note that if you already know the upper limit of prime number you want,
    /// you can use `new_up_to(x)`, it will be faster.
    pub fn new() -> Self {
        Self {
            p: 1,
            previous_primes: Vec::new(),
            limit: None,
        }
    }

    /// Create a new iterator, producing prime numbers up to a limit.
    pub fn new_up_to(x: usize) -> Self {
        Self {
            p: 1,
            previous_primes: Vec::new(),
            limit: Some(x),
        }
    }
}

// Implement `Iterator` for `Primes`.
impl Iterator for Primes {
    type Item = usize;

    // Define how to compute the next prime number.
    fn next(&mut self) -> Option<Self::Item> {
        if self.p < 3 {
            // Special cases for the first primes : 2 and 3 are primes. After
            // that we can just skip even numbers.
            self.p += 1;
        } else {
            loop {
                self.p += 2; // Skip even numbers

                // Check if the number is divisible by any previous prime
                let mut divisible = false;
                for pp in self.previous_primes.iter().rev() {
                    if self.p % pp == 0 {
                        divisible = true;
                        break;
                    }
                }

                if !divisible {
                    // We found the next prime !
                    break;
                }
            }
        }

        match self.limit {
            Some(lim) => {
                if self.p > lim {
                    return None;
                }

                // When we have a limit, we don't need to store past primes that
                // are over the square root of the limit, it will be faster
                if self.p as f64 <= (lim as f64).sqrt() {
                    self.previous_primes.push(self.p);
                }
                Some(self.p)
            }
            None => {
                // When we don't have limit, store every past primes for the next step
                self.previous_primes.push(self.p);
                Some(self.p)
            }
        }
    }
}

/// Compute the list of divisors for the given number.
///
/// # Examples
/// ```
/// let divisors = reuler::utils::get_divisors(8);
///
/// assert_eq!(divisors.len(), 4);
/// assert!(divisors.contains(&1));
/// assert!(divisors.contains(&2));
/// assert!(divisors.contains(&4));
/// assert!(divisors.contains(&8));
/// ```
pub fn get_divisors(x: usize) -> HashSet<usize> {
    let mut divisors = HashSet::new();
    let mut i = 1;

    while i as f64 <= (x as f64).sqrt() {
        if x % i == 0 {
            divisors.insert(i);
            divisors.insert(x / i);
        }

        i += 1;
    }
    divisors
}

/// Compute the list of **proper** divisors for the given number.
///
/// # Examples
/// ```
/// let divisors = reuler::utils::get_proper_divisors(8);
///
/// assert_eq!(divisors.len(), 3);
/// assert!(divisors.contains(&1));
/// assert!(divisors.contains(&2));
/// assert!(divisors.contains(&4));
/// ```
pub fn get_proper_divisors(x: usize) -> HashSet<usize> {
    let mut divisors = get_divisors(x);
    divisors.remove(&x);
    divisors
}

/// Structure used to deal with arbtrarily large numbers.
///
/// # Note
/// For now, only addition between two BigInt, and multiplication-assign with
/// a `usize` is supported.
///
/// # Examples
/// ```
/// let mut x = reuler::utils::BigInt::from(128);
/// let y = reuler::utils::BigInt::new();  // It's 0
/// x = &x + &y;
/// x *= 2;
///
/// assert_eq!(x.to_string(), String::from("256"));
/// ```
#[derive(Debug, PartialEq)]
pub struct BigInt {
    pub digits: Vec<usize>,
}

impl BigInt {
    /// Create a new BigInt, starting at 0. To create a BigInt from an existing
    /// number, use `BigInt::from()` instead.
    pub fn new() -> Self {
        Self { digits: vec![0] }
    }

    /// Create a new BigInt from the given number.
    pub fn from(mut x: usize) -> Self {
        let mut x_digits = Vec::new();
        while x != 0 || x_digits.len() == 0 {
            // build the number, digit by digit
            x_digits.push(x % 10);

            x = x / 10;
        }
        Self { digits: x_digits }
    }

    /// Function to get the number as a string.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for d in self.digits.iter().rev() {
            s.push_str(&d.to_string());
        }
        s
    }
}

impl ops::Add<&BigInt> for &BigInt {
    type Output = BigInt;

    /// Overload the addition for BigInt.
    fn add(self, x: &BigInt) -> Self::Output {
        // Clone the digits of self, we will update them and create a new BigInt
        let mut digits = self.digits.clone();

        // Make both digits the same length
        while digits.len() < x.digits.len() {
            digits.push(0);
        }

        // Add the digits together, one-by-one
        let mut carry_over = 0;
        for i in 0..digits.len() {
            let x_digit = if i < x.digits.len() { x.digits[i] } else { 0 };
            let digit_result = digits[i] + x_digit + carry_over;

            // Update the current digit
            digits[i] = digit_result % 10;

            // Update carry over
            carry_over = digit_result / 10;
        }

        while carry_over != 0 {
            // We have a carry over left, add a new digit
            digits.push(carry_over % 10);

            carry_over = carry_over / 10;
        }

        BigInt { digits: digits }
    }
}

impl ops::MulAssign<usize> for BigInt {
    /// Overload the multiplication-assign for BigInt.
    fn mul_assign(&mut self, x: usize) {
        // Multiply each digit, one-by-one
        let mut carry_over = 0;
        for i in 0..self.digits.len() {
            let digit_result = self.digits[i] * x + carry_over;

            // Update the current digit
            self.digits[i] = digit_result % 10;

            // Update carry over
            carry_over = digit_result / 10;
        }

        while carry_over != 0 {
            // We have a carry over left, add a new digit
            self.digits.push(carry_over % 10);

            carry_over = carry_over / 10;
        }

        // If we multiply by 0, we have to remove some digits to have only a single 0
        while self.digits.len() > 1 && self.digits[self.digits.len() - 1] == 0 {
            self.digits.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_addition_same_number_of_digits() {
        assert_eq!(
            &BigInt::from(125) + &BigInt::from(988),
            BigInt::from(125 + 988)
        );
    }

    #[test]
    fn test_bigint_addition_more_digits() {
        assert_eq!(&BigInt::from(125) + &BigInt::from(7), BigInt::from(125 + 7));
    }

    #[test]
    fn test_bigint_addition_less_digits() {
        assert_eq!(
            &BigInt::from(56) + &BigInt::from(4852),
            BigInt::from(56 + 4852)
        );
    }

    #[test]
    fn test_bigint_new() {
        assert_eq!(BigInt::new(), BigInt::from(0));
    }

    #[test]
    fn test_bigint_multiplication_1() {
        let mut big_int = BigInt::from(75);
        big_int *= 3;
        assert_eq!(big_int, BigInt::from(75 * 3));
    }

    #[test]
    fn test_bigint_multiplication_2() {
        let mut big_int = BigInt::from(3);
        big_int *= 75;
        assert_eq!(big_int, BigInt::from(3 * 75));
    }

    #[test]
    fn test_bigint_multiplication_by_zero() {
        let mut big_int = BigInt::from(9888);
        big_int *= 0;
        assert_eq!(big_int, BigInt::new());
    }
}
