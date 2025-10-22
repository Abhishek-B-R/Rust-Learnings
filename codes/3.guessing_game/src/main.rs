use std::io::stdin;

fn main() {
    println!("Guess the number");
    println!("Enter a number");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Failed to read nuber");

    println!("Guessed number is {guess}");
}
