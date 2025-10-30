fn main() {
    let mut x = String::from("hello");
    let mut y = &x;

    let binding = String::from("hey there");
    y = &binding;
    change_value(&mut x);
    
    println!("Here, x is {} and y is {}", x, y);
}

fn change_value(s : &mut String){
    s.push_str(" world");
}

// fn main() {
//     let _reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }