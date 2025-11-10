use std::io;
use std::io::std

fn main() {
    println!("Hello, world!");
    let mut x = String::new();

    iostdin().read_line(&mut x).expect("error reading data");
    print!("{}",x);
    io::stdout().flush();
}
