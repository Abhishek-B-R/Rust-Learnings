use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let mut x = String::new();

    stdin().read_line(&mut x).expect("error reading data");
    println!("{}",x)
}
