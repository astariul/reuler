/// Decrypt a given list of values with a given password with XOR decryption. 
fn decrypt(crypted_message: &Vec<u8>, password: &Vec<u8>) -> String {
    let mut message = String::new();
    for i in 0..crypted_message.len() {
        let c = crypted_message[i] ^ password[i % password.len()];
        message.push(c as char);
    }
    message
}

/// Decipher the given ciphered text with XOR decryption, with a password made
/// of lowercase letter of the given len.
///
/// # Notes
/// Instead of relying on the most common English words to find if the text is
/// properly decrypted, count the number of words. Indeed, if the text is not
/// properly decrypted, space will not be properly decrypted and we will not
/// have several words. If the text is properly decrypted, the space characters
/// will be properly decrypted and we will have several words. 
fn decipher(ciphered_text: &str, password_len: usize) -> usize {
    // Read the ASCII values from the ciphered text
    let mut ciphered_values = Vec::new();
    for v in ciphered_text.split(',') {
        let v: u8 = v.parse().unwrap();
        ciphered_values.push(v);
    }

    // Generate the possible passwords & try to decrypt with them
    let mut password = vec!['a' as u8; password_len];
    let mut best_n_words = 0;
    let mut best_message = String::new();
    'try_pass: loop {
        // Decrypt the message with this password
        let message = decrypt(&ciphered_values, &password);
        let words: Vec<&str> = message.split_whitespace().collect();
        
        if words.len() > best_n_words {
            best_n_words = words.len();
            best_message = message;
        }

        // For the next iteration, try the next password
        let mut i = password.len() - 1;
        while password[i] == ('z' as u8) {
            if i == 0 {
                break 'try_pass;
            }
            password[i] = 'a' as u8;
            i -= 1;
        }
        password[i] += 1;
    }

    // Return the sum of uncrypted ASCII values
    best_message.chars().map(|c| c as usize).sum()
}

/// Solve the problem #59 and return the solution.
pub fn solve() -> String {
    let ciphered_text = include_str!("data/cipher.txt");
    decipher(ciphered_text, 3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_single_char() {
        let original = String::from("This is a test");
        let original_code = original.chars().map(|c| c as u8).collect();
        let password = vec!['b' as u8];
        let encrypted = decrypt(&original_code, &password);
        let encrypted_code = encrypted.chars().map(|c| c as u8).collect();
        assert_eq!(decrypt(&encrypted_code, &password), original);
    }

    #[test]
    fn test_encrypt_decrypt_multiple_chars() {
        let original = String::from("This is a test");
        let original_code = original.chars().map(|c| c as u8).collect();
        let password = vec!['b' as u8, 'x' as u8, 'f' as u8, 'w' as u8];
        let encrypted = decrypt(&original_code, &password);
        let encrypted_code = encrypted.chars().map(|c| c as u8).collect();
        assert_eq!(decrypt(&encrypted_code, &password), original);
    }
}
