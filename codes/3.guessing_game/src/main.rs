use std::io::stdin;
use std::cmp::Ordering;

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

    let guess : i32 = guess.trim().parse().expect("Please enter a number"); 
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
    
    println!("Guessed number is {guess}");
}
