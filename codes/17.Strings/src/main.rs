fn main() {
    // let mut x = String::from("asdf");
    // x.push('🤣');
    // println!("{}",x );

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}",s3);

    let hello = "hi there this is abhishek";

    for word in hello.split_whitespace() {
        println!("{}", word);
    }
}