fn main() {
    println!("Hello, world!");

    another_function(5,'a');
}

fn another_function(x:i32, c:char) {
    println!("Another function with int : {x} and char : {c}");
}