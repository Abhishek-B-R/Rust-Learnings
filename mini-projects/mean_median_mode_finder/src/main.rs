// use std::io::stdin;
use std::collections::HashMap;

fn main(){
    let mut list = vec![9,7,4,2,5,5,1,8,3,6];
    let len = list.len();

    // mean calculation
    let mut mean = 0;
    for i in &list{
        mean += i;
    }
    mean = mean / len;
    println!("{}", mean);

    // median calculation
    let median : usize;
    list.sort();
    if len % 2 != 0 {
        median = list[len / 2];
    }else{
        median = (list[len / 2] + list[len / 2 + 1]) / 2;
    }
    println!("{}", median);

    // mode calculation
    for i in &list {
        
    }
}
