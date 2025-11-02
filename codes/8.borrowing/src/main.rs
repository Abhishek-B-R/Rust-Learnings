fn main() {
    let mut x = String::from("hello");
    let y;

    let binding = String::from("hey there");
    y = &binding;
    print_val(&x);
    change_value(&mut x);
    print_val(&x);
    println!("Here, x is {} and y is {}", x, y);
}

fn change_value(s : &mut String){
    s.push_str(" world");
}

fn print_val(s : &String){
    println!("{}",s);
}

// fn main() {
//     let _reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }