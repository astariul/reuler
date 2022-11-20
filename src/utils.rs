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
}

impl Primes {
    pub fn new() -> Self {
        Self {
            p: 1,
            previous_primes: vec![2],
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
                self.p += 2;    // Skip even numbers

                // Check if the number is divisible by any previous prime
                let mut divisible = false;
                for pp in &self.previous_primes {
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

        self.previous_primes.push(self.p);
        Some(self.p)
    }
}
