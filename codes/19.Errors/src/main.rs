// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");
    
//     let file = match greeting_file_result{
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(file) => file,
//                 Err(_error) => panic!("Couldn't create a new file"),
//             },
//             _ => panic!("Problem opening the file"),
//         }
//     };

//     println!("{:?}", file);
// }

// use std::fs::File;

// fn main() {
//     let greeting_file = File::open("hello.txt").expect("Error opening file");
//     println!("{greeting_file:?}");
// }

use std::fs::{self,File};
use std::io::{self, Read};

fn main() {
    match read_username_from_file_simplest(){
        Ok(data) => println!("{}", data),
        _ => ()
    }
}

// same as below function but a bit more longer since I've not used "?" operator
fn read_username_from_file() -> Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simpler() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_more_simpler() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simplest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}