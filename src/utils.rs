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

/// Compute the list of divisors for the given number.
///
/// # Examples
/// ```
/// assert!(!reuler::utils::is_prime(1));
/// assert!(reuler::utils::is_prime(2));
/// assert!(reuler::utils::is_prime(3));
/// assert!(!reuler::utils::is_prime(8));
/// ```
pub fn is_prime(x: usize) -> bool {
    match x {
        1 => false,
        2 => true,
        _ if x % 2 == 0 => false,
        _ => {
            let divisors = get_divisors(x);
            divisors.len() == 2
        }
    }
}

/// Compute the list of divisors for the given number.
///
/// # Examples
/// ```
/// assert!(!reuler::utils::is_prime_with_neg(8));
/// assert!(!reuler::utils::is_prime_with_neg(-8));
/// assert!(reuler::utils::is_prime_with_neg(3));
/// assert!(reuler::utils::is_prime_with_neg(-3));
/// assert!(!reuler::utils::is_prime_with_neg(8));
/// ```
pub fn is_prime_with_neg(x: isize) -> bool {
    let x = if x < 0 { -x } else { x };
    is_prime(usize::try_from(x).unwrap())
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

/// Compute the Greatest Common Factor (GCF) between 2 numbers.
///
/// # Examples
/// ```
/// assert_eq!(reuler::utils::gcf(18, 27), 9);
/// assert_eq!(reuler::utils::gcf(60, 48), 12);
/// assert_eq!(reuler::utils::gcf(8, 9), 1);
/// ```
pub fn gcf(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

/// Function that extract the digits of a given number, in a given base.
///
/// # Notes
/// This function returns the digits in reverse order. If you need to get the
/// digits in "natural" order, call `.reverse()` on the results.
///
/// # Examples
/// Get the digits of a number in base 10 :
/// ```
/// let digits = reuler::utils::digits_of_base(127, 10);
///
/// assert_eq!(digits.len(), 3);
/// assert_eq!(digits[0], 7);
/// assert_eq!(digits[1], 2);
/// assert_eq!(digits[2], 1);
/// ```
///
/// Get the digits of a number in base 10, in the "natural" order :
/// ```
/// let mut digits = reuler::utils::digits_of_base(127, 10);
/// digits.reverse();
///
/// assert_eq!(digits.len(), 3);
/// assert_eq!(digits[0], 1);
/// assert_eq!(digits[1], 2);
/// assert_eq!(digits[2], 7);
/// ```
///
/// Get the digits of a number in base 2 :
/// ```
/// let digits = reuler::utils::digits_of_base(10, 2);
///
/// assert_eq!(digits.len(), 4);
/// assert_eq!(digits[0], 0);
/// assert_eq!(digits[1], 1);
/// assert_eq!(digits[2], 0);
/// assert_eq!(digits[1], 1);
/// ```
pub fn digits_of_base(x: usize, base: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut remain = x;
    while remain > 0 || digits.len() == 0 {
        digits.push(remain % base);
        remain = remain / base;
    }
    digits
}

/// Function that extract the digits of a given number.
///
/// # Notes
/// This function returns the digits in reverse order. If you need to get the
/// digits in "natural" order, call `.reverse()` on the results.
///
/// # Examples
/// Get the digits of a number :
/// ```
/// let digits = reuler::utils::digits_of(127);
///
/// assert_eq!(digits.len(), 3);
/// assert_eq!(digits[0], 7);
/// assert_eq!(digits[1], 2);
/// assert_eq!(digits[2], 1);
/// ```
///
/// Get the digits of a number, in the "natural" order :
/// ```
/// let mut digits = reuler::utils::digits_of(127);
/// digits.reverse();
///
/// assert_eq!(digits.len(), 3);
/// assert_eq!(digits[0], 1);
/// assert_eq!(digits[1], 2);
/// assert_eq!(digits[2], 7);
/// ```
pub fn digits_of(x: usize) -> Vec<usize> {
    digits_of_base(x, 10)
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
    pub fn from(x: usize) -> Self {
        Self {
            digits: digits_of(x),
        }
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

/// Function that check if an iterable is a palindrome or not.
///
/// # Examples
/// A number that is a palindrome :
/// ```
/// let digits = reuler::utils::digits_of(125521);
///
/// assert!(reuler::utils::is_palindrome(digits));
/// ```
///
/// A number that is **not** a palindrome :
/// ```
/// let digits = reuler::utils::digits_of(129);
///
/// assert!(!reuler::utils::is_palindrome(digits));
/// ```
pub fn is_palindrome<T>(x: Vec<T>) -> bool
where
    T: std::cmp::PartialEq,
{
    for i in 0..x.len() / 2 {
        if x[i] != x[x.len() - i - 1] {
            return false;
        }
    }
    true
}

/// Compute all possible permutations of the given array.
///
/// # Examples
/// ```
/// let permutations = reuler::utils::permutations_of(vec![1, 2, 3]);
///
/// assert_eq!(permutations.len(), 6);
/// assert_eq!(permutations[0], vec![3, 2, 1]);
/// assert_eq!(permutations[1], vec![2, 3, 1]);
/// assert_eq!(permutations[2], vec![2, 1, 3]);
/// assert_eq!(permutations[3], vec![3, 1, 2]);
/// assert_eq!(permutations[4], vec![1, 3, 2]);
/// assert_eq!(permutations[5], vec![1, 2, 3]);
/// ```
pub fn permutations_of<T>(array: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    // Base case
    if array.len() < 2 {
        return vec![array];
    }

    // Recursive case
    let mut sub_case = array.clone();
    let first_element = sub_case.pop().unwrap();
    let mut permutations = Vec::new();

    for p in permutations_of(sub_case) {
        for i in 0..p.len() + 1 {
            let mut new_p = p.clone();
            new_p.insert(i, first_element);
            permutations.push(new_p);
        }
    }
    permutations
}

/// Convert a list of digits into a number.
///
/// # Notes
/// The number created uses the digits in reverse order. This is a feature, not
/// a bug, to be able to use this function together with the function
/// `digits_of()` (which returns the digits in reverse order).
///
/// # Examples
/// ```
/// assert_eq!(reuler::utils::digits_to_number(vec![1, 2, 3]), 321);
/// ```
pub fn digits_to_number(digits: Vec<usize>) -> usize {
    let mut number = 0;
    let mut m = 1;
    for d in digits.iter() {
        number += d * m;
        m *= 10;
    }
    number
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

    #[test]
    fn test_digit_of_0() {
        let digits = digits_of(0);
        assert_eq!(digits.len(), 1);
        assert_eq!(digits[0], 0);
    }

    #[test]
    fn test_empty_permutation() {
        assert_eq!(permutations_of(Vec::<usize>::new()).len(), 1);
        assert_eq!(permutations_of(Vec::<usize>::new())[0].len(), 0);
    }

    #[test]
    fn test_permutation_single_element() {
        assert_eq!(permutations_of(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn test_empty_digits() {
        assert_eq!(digits_to_number(Vec::new()), 0);
    }
}
