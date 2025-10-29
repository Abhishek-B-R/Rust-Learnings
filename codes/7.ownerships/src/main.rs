fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(" world");
    takes_ownership(s1.clone());
    println!("s1 is {}",s1);
    println!("s2 is {}",s2);
}

fn takes_ownership(s: String){
    println!("{s}");
}