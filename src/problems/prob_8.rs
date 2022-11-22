/// Compute the greatest product of the n adjacent digits in the 
/// problem-specific 1000-digit number.
fn greatest_product_adjacent(n: usize) -> u64 {
    // Some preprocessing
    let number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let number_chars = number.chars();
    let mut digits = Vec::new();

    for c in number_chars {
        let d = u64::from(c.to_digit(10).unwrap());
        digits.push(d);
    }

    // The algorithm itself
    let mut i = 0;
    let mut curr_sum = 1;
    let mut max_sum = 0;

    // Compute the first sum
    while i < n {
        curr_sum *= digits[i];
        i += 1;
    }

    if curr_sum > max_sum {
        max_sum = curr_sum;
    }

    // Keep iterating, updating the sum
    while i < number.len() {
        // Special case of 0 : it nullify the product, so just skip it entirely
        if digits[i] == 0 {
            i += n;
            continue;
        }

        // Advance by one position : update the current sum with the new digit
        curr_sum *= digits[i];

        // Since we advanced, pop the last digit, but be careful of division by zero !
        if digits[i - n] == 0 {
            // Since a 0 cancel everything, we need to recompute the sum
            curr_sum = 1;
            for j in i - n + 1..i + 1 {
                curr_sum *= digits[j];
            }
        } else {
            // Otherwise, a simple division is fine
            curr_sum /= digits[i - n];
        }

        if curr_sum > max_sum {
            max_sum = curr_sum;
        }

        i += 1;
    }

    max_sum
}

/// Solve the problem #8 and return the solution.
pub fn solve() -> String {
    greatest_product_adjacent(13).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        assert_eq!(greatest_product_adjacent(4), 5832);
    }
}
