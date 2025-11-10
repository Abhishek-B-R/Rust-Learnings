use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Calculator App in Rust");

    loop {
        println!("Press 'q' to exit");
        let mut x = String::new();
        print!("Enter your first number : ");
        io::stdout().flush().unwrap();÷
        io::stdin()
            .read_line(&mut x)
            .expect("Unable to read user input");
        if x.trim() == "q"{
            break;
        }

        let x : i128 = x.trim().parse().expect("Here 'x' is a invalid number");
        
        print!("Enter the operation you wanna do, like +,-,*,/,% : ");
        let mut operation = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut operation)
            .expect("Unable to read what operation you wanna perform");
    
        let mut y = String::new();
        print!("Enter your second number : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut y)
            .expect("Unable to read user input");
        if y.trim() == "q"{
            break;
        }
        let y : i128 = y.trim().parse().expect("Here 'y' is a invalid number");

        match operation.trim().as_ref() {
            "+" => println!("{}",x+y),
            "-" => println!("{}",x-y),
            "*" => println!("{}",x*y),
            "/" => {
                if y != 0{
                    println!("{}",x/y)
                } else {
                    println!("Math Error")
                }
            },
            "%" => {
                if y != 0{
                    println!("{}",x%y)
                } else {
                    println!("Math Error")
                }
            },
            "q" => break,
            _ => println!("Seems like you entered some value or expressions wrong, try again :)"),
        };
    }
}
