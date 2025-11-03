#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other : &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width : 30,
        height : 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of rectangle is {} and width is {}",
        rect1.area(), rect1.width()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// struct Point {
//     a: u32,
// }
// impl Point {
//     fn distance(&self) -> u32 {
//         self.a
//     }
// }
// fn main() {
//     let p1 = Point { a: 5 };     // a normal value
//     let p2 = &Point { a: 10 };   // a reference to a Point
//     println!("{}", p1.distance());  // works directly on value
//     println!("{}", p2.distance());  // also works, even though p2 is a reference
// }
