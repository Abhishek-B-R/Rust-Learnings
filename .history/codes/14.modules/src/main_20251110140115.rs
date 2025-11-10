use std::io::stdin;
use s

fn main() {
    println!("Hello, world!");
    let mut x = String::new();

    stdin().read_line(&mut x).expect("error reading data");
    print!("{}",x);
    io::stdout().flush();
}
