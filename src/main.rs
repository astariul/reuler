use rustere::is_odd;
use std::io;

fn main() {
    let guess_nb: usize;

    loop {
        println!("Enter a number to know if odd.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess_nb = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

    match is_odd(guess_nb) {
        true => println!("{guess_nb} is odd."),
        false => println!("{guess_nb} is even."),
    }
}
