use std::fs;
use std::ops;

fn main() {
    let file_path = String::from("./input");
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    chall_one(&contents);
    chall_two(&contents);

}

fn chall_one(data: &str) {
    let mut elfselect = 0;
    let mut firstdash: usize = 0 ;
    let mut seconddash: usize = 0 ;
    let mut comma: usize = 0;
    let mut total_pairs: i32 = 0;
    for line in data.lines() {
        let linelen: usize = line.len();
        for (i, ch) in line.chars().enumerate(){
            if ch == '-' {
                if elfselect == 0 {firstdash = i; elfselect = 1;}
                else {seconddash = i;elfselect = 0;}
            } else if ch == ','{
                comma = i;
            }
        }
        let firstelf: ops::Range<i32> = *&line[0..firstdash].parse::<i32>().unwrap()..*&line[firstdash+1..comma].parse::<i32>().unwrap()+1;
        let secondelf: ops::Range<i32> = *&line[comma+1..seconddash].parse::<i32>().unwrap()..*&line[seconddash+1..linelen].parse::<i32>().unwrap()+1;
        if firstelf.len() > secondelf.len() {
            if check_subset(secondelf, firstelf){
                total_pairs += 1;
            }
        } else {
            if check_subset(firstelf, secondelf){
                total_pairs += 1;
            }
        }
    }
    println!("{}",total_pairs);
}

fn chall_two(data: &str) {
    let mut elfselect = 0;
    let mut firstdash: usize = 0 ;
    let mut seconddash: usize = 0 ;
    let mut comma: usize = 0;
    let mut total_pairs: i32 = 0;
    for line in data.lines() {
        let linelen: usize = line.len();
        for (i, ch) in line.chars().enumerate(){
            if ch == '-' {
                if elfselect == 0 {firstdash = i; elfselect = 1;}
                else {seconddash = i;elfselect = 0;}
            } else if ch == ','{
                comma = i;
            }
        }
        let firstelf: ops::Range<i32> = *&line[0..firstdash].parse::<i32>().unwrap()..*&line[firstdash+1..comma].parse::<i32>().unwrap()+1;
        let secondelf: ops::Range<i32> = *&line[comma+1..seconddash].parse::<i32>().unwrap()..*&line[seconddash+1..linelen].parse::<i32>().unwrap()+1;
        if check_overlaps(secondelf, firstelf){
            total_pairs += 1;
        }
    }
    println!("{}",total_pairs);
}

fn check_overlaps(smaller: ops::Range<i32>, bigger: ops::Range<i32>) -> bool {
    for task in smaller {
        if bigger.contains(&task){
            return true
        }
    }
    return false
}

fn check_subset(smaller: ops::Range<i32>, bigger: ops::Range<i32>) -> bool {
    for task in smaller {
        if !bigger.contains(&task){
            return false
        }
    }
    return true
}

// fn create_vec_range()
