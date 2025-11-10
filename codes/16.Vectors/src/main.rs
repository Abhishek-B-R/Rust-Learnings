fn main() {
    let mut v1 : Vec<i32> = Vec::new();
    let v2 = vec![1,2,3,4,5];

    v1.push(5);

    let third_elem : &i32 = &v2[2];

    println!("{}",third_elem);

    for i in &mut v1 {
        *i += 50;
        println!("{i}");
    }

    for i in &v2 {
        println!("{i}");
    }
}
