use std::io::stdin;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Enter a number");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read nuber");

    println!("Guessed number is {guess}");
    
}
