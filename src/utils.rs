use std::collections::HashSet;

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
