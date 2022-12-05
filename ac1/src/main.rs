use std::io::{stdout, BufWriter};
use std::env;
use std::fs;


fn main() {
    // --snip--
    let file_path = String::from("./input");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let content_lines = contents.lines();
    let mut v: Vec<i32> = Vec::new();
    let mut total: i32 = 0;
    for l in content_lines {
        if !l.is_empty() {
            let val: i32  = l.parse().unwrap();
            total += val;
        } else {
            v.push(total);
            total = 0;
        }
    }
    let max_val = &v.iter().max().unwrap();
    println!("Max: {}", max_val);

    v.sort();
    let len = v.len();
    let top_tree = &v[len - 3..len];
    let mut sum = 0;
    for top_cals in top_tree {
        sum += top_cals;
    }
    println!("{}",sum)
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}