use std::{fs, hash};
use std::collections::{vec_deque, HashMap};
fn main() {
    let file_path: String = String::from("./input");
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut lines = Vec::<String>::new();
    let mut piles: HashMap<usize, Vec<char>> = HashMap::new();
    let mut piles_keys: HashMap<usize, usize> = HashMap::new();
    let mut init_lines = Vec::new();
    let mut commands = Vec::new();
    let mut kee: usize = 1;
    let mut commandsornot: bool = false;
    for line in contents.lines() {
        if commandsornot{
            commands.push(line);
        } else {
            if line.len() != 0 {
                init_lines.push(line);
            } else {
                commandsornot = true;
            }
        } 
    }
    let columns = &init_lines[init_lines.len() - 1];
    println!("{}", columns)
}
//     let len_init: usize = init_lines.len();
//     for line in contents.lines
//     for (j, c) in line.bytes().enumerate(){
//         if c != b' '{
//             let ival: usize = usize::from(c);
//             piles.insert(ival, Vec::<char>.new());
//             piles_keys.insert(ival, j);
//             kee += 1;
//         }
//     }

// }


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}