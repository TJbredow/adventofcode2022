use std::fs;

fn main() {
    let file_path = String::from("./input");
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    chall_two(&contents);
}
fn calculate_value(present: u8) -> i32 {
    let present: i32 = i32::from(present);
    if present > 96 {
        return present - 96
    } else {
        return present - 38
    }
}
fn chall_one(data: &String) {
    let sacks = data.lines();
    let mut total: i32 = 0;
    for sack in sacks {
        let sack_len: usize = sack.len();
        let sack1: &str = &sack[0..sack_len/2];
        let sack2: Vec<u8> = Vec::from(&sack[sack_len/2..sack_len]);
        for present in sack1.bytes() {
            if sack2.contains(&present){
                let presentchar: char = char::from(present);
                let prio: i32 = calculate_value(present);
                total += prio;
                break;

            } else {}
        }
    }
    println!("{}",total);
}

fn chall_two(data: &String) {
    let sacks = data.lines();
    let mut total: i32 = 0;
    let mut gi: usize = 0;
    let mut sacksvec: Vec<String> = Vec::new();
    for sack in sacks {
        sacksvec.push(sack.to_string());
    } 
    let veclen: usize = sacksvec.len();
    while gi < veclen {
        println!("here");
        let gnome1 = &sacksvec[gi].to_string();
        let gnome2 = &sacksvec[gi+1].to_string();
        let gnome3 = &sacksvec[gi+2].to_string();
        total += evaluate_value(gnome1, gnome2, gnome3);
        gi += 3;
    }
    println!("{}",total)
}

fn evaluate_value(g1: &String, g2: &String, g3: &String) -> i32 {
    println!("{} -- {} -- {}", g1, g2, g3);
    let gnome2: Vec<u8> = Vec::from(g2.to_string());
    let gnome3: Vec<u8> = Vec::from(g3.to_string());
    for medallion in g1.bytes() {
        if gnome2.contains(&medallion) && gnome3.contains(&medallion) {
            println!("{}",medallion);
            return calculate_value(medallion);
        }
    }
    return 0;
}