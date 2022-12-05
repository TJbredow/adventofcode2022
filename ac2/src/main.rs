use std::fs;

fn main() {
    let file_path = String::from("./input");
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let content_lines = contents.lines();
    let mut total_score: i32 = 0;
    for l in content_lines {
        let char_vec: Vec<char> = l.chars().collect();
        let p: char = char_vec[2];
        let o: char = char_vec[0];
        total_score += calculate_round(p, o);
    }

    println!("{}",total_score);
}

fn calculate_round(player: char, opponent: char) -> i32{
    if player == 'X' {
        if opponent == 'A' {return 0 + 3}
        else if opponent == 'B' {return 0 + 1}
        else {return 0 + 2}
    }
    else if player == 'Y' {
        if opponent == 'A' {return 1 + 3}
        else if opponent == 'B' {return 2 + 3}
        else {return 3 + 3}
    }
    else {
        if opponent == 'A' {return 6 + 2}
        else if opponent == 'B' {return 6 + 3}
        else {return 6 + 1}
    }
}
