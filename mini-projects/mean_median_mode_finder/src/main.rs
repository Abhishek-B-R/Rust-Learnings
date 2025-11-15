// use std::io::stdin;
use std::collections::HashMap;

fn main(){
    let mut list = vec![9,7,4,2,7,5,1,8,3,6];
    let len = list.len();

    // mean calculation
    let mut mean = 0;
    for i in &list{
        mean += i;
    }
    let mean : f64 = mean as f64/ len as f64;
    println!("Mean - {}", mean);

    // median calculation
    let median : usize;
    list.sort();
    if len % 2 != 0 {
        median = list[len / 2];
    }else{
        median = (list[len / 2] + list[len / 2 + 1]) / 2;
    }
    println!("Median - {}", median);

    // mode calculation
    let mut map : HashMap<usize,usize> = HashMap::new();
    for i in list {
        let x = map.get(&i).copied().unwrap_or(0)+1;
        map.insert(i,x);
    }

    let mut map_items : Vec<(_,_)> = map.iter().collect();
    map_items.sort_by(|a,b| b.1.cmp(a.1));

    let mode = &map_items[0].0;
    println!("Mode - {}", mode);
}
