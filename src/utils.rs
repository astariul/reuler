pub struct Fibonacci {
    pub curr: usize,
    pub next: usize,
}

// Implement `Iterator` for `Fibonacci`.
impl Iterator for Fibonacci {
    type Item = usize;

    // Define how to compute the next step.
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

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
