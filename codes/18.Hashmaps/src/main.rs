use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(String::from("Blue"),10);
    map.insert(String::from("Red"),40);

    let x = map.get("Blue").copied().unwrap_or(0);
    println!("{}", x);
}