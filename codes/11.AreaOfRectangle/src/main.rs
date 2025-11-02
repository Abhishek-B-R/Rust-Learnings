// #[derive(Debug)]
// struct Rectangle{
//     height : u32,
//     width : u32,
// }

// fn main() {
//     let rect1 = Rectangle{
//         height : 10,
//         width : 20,
//     };

//     // println!("area - {}",area(&rect1));

//     println!("rectangle is {rect1:#?}");
// }

// fn area(r : &Rectangle) -> u32{
//     r.height * r.width
// }


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}