/// Compute the sum of the numbers which are equal to the sum of the factorial
/// of their digits.
fn sum_digit_factorials() -> usize {
    // First, precompute factorial of digits
    let mut d_factorial: Vec<usize> = vec![1];
    let mut curr_factorial = 1;
    for i in 1..10 {
        curr_factorial *= i;
        d_factorial.push(curr_factorial);
    }

    // Compute the maximum number of digits (upper limit)
    let mut max_n_digits: u32 = 2;
    let fact9 = u32::try_from(d_factorial[9]).unwrap();
    loop {
        if fact9 * (max_n_digits + 1) < 10_u32.pow(max_n_digits + 1) {
            break;
        }
        max_n_digits += 1;
    }

    println!("{max_n_digits}");

    // Compute the sum
    let mut sum = 0;
    for n in 10..10_usize.pow(max_n_digits) {
        let mut remain = n;
        let mut factorial_sum = 0;
        while remain > 0 {
            factorial_sum += d_factorial[remain % 10];
            remain = remain / 10;
        }

        if factorial_sum == n {
            sum += n;
        }
    }
    sum
}

/// Solve the problem #34 and return the solution.
pub fn solve() -> String {
    sum_digit_factorials().to_string()
}
