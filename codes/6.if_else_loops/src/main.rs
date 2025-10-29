// fn main (){
//     let x = 3;

//     if (x == 3) {
//         println!("hi there");
//     } else {
//         println!("hello there")
//     }
// }


// fn main() {
//     let mut x = 0;
//     'outer: loop {
//         println!("outer loop");
//         loop {
//             println!("inner loop");
//             x += 1;
//             if x == 5 {
//                 break 'outer;
//             } else {
//                 break;
//             }
//         }
//     }
// }

// fn main() {
//     let arr = [1,2,3,4,5];
//     let mut index = 0;

//     while index < arr.len() {
//         println!("Element at index {}: {}", index, arr[index]);
//         index += 1;
//     }
// }


// fn main() {
//     let arr = [1,2,3,4,5];
//     for (i, element) in arr.iter().enumerate() {
//         println!("Element at index {}: {}", i, element);
//     }
// }

fn main() {
    for x in 1..4{
        println!("{x}");
    }
}