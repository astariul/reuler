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
