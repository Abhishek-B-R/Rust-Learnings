use std::io;
use std::io::StdOut;

fn main() {
    println!("Hello, world!");
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("error reading data");
    print!("{}",x);
    io::stdout().flush();
}
