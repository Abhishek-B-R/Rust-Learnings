fn main() {
    let mut s = String::from("hello there");
    let first = first_word(&s); // borrowing and recieve first work

    // s.clear(); // Gives error
    println!("{}", first); //print it
}

fn first_word(s : &str) -> &str{ // take immutable reference of string and return string
    let bytes = s.as_bytes(); // split it into bytes array

    for (i, &byte) in bytes.iter().enumerate() { // iterate the bytes and enumarate it -> gives index and 
        if byte == b' ' { // here b' ' means the bytecode of space(' ')
            return &s[0..i]; // return the first part of the string, bia reference obv
        }
    }

    return &s[..]; // if no space found, return entire s
}