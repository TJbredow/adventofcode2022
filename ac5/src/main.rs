use std::{fs, hash};
use std::collections::{vec_deque, HashMap};
use std::ops::Range;
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
    // Build the initial state into mmemory
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
    let columns = init_lines.pop().unwrap();
    println!("{}", columns);
    for (j, c) in columns.bytes().enumerate(){
        if c != b' '{
            let ival: usize = (usize::from(c) - 48);
            piles.insert(ival, Vec::<char>::new());
            piles_keys.insert(ival, j);
            kee += 1;
        }
    }
    for line in init_lines {
        let line_b = line.as_bytes();
        for (j, mut val) in &mut piles{
            let refer = piles_keys[&j];
            if line_b[refer] != b' '{
                val.push(char::from(line_b[refer]));
            }
        }
    }
    let mut i: usize = 1;
    // Run the logic
    println!("    ");
    for line in commands {
        // Changes the "from" list first, the function pulls the borrow out of scope so the compiler knows when the borrow is released
        mutate_from(&mut piles, line);
        let splitline = line.split(" ").collect::<Vec<&str>>();
        let cfrom: usize = splitline[3].parse::<usize>().unwrap();
        let cto: usize = splitline[5].parse::<usize>().unwrap();
        let howmanyint: usize = splitline[1].parse::<usize>().unwrap();
        let howmany: Range<usize> = 0..howmanyint;
        let mut fromcolumn = piles.get_mut(&cfrom).unwrap();
        let howmanyleft: Range<usize> = howmanyint..fromcolumn.len();
        *fromcolumn = fromcolumn[howmanyleft].to_vec();
    }
    let mut i: usize = 1;
    // Print out the cargo status :)
    while i < 10 {
        println!("{}: {:?}", i, &piles[&i].to_vec());
        i += 1;
    }
}
fn mutate_from(stacks: &mut HashMap<usize, Vec<char>>, line: &str) {
    // println!("{}",&line);
    let splitline = line.split(" ").collect::<Vec<&str>>();
    let cfrom: usize = splitline[3].parse::<usize>().unwrap();
    let cto: usize = splitline[5].parse::<usize>().unwrap();
    let howmanyint: usize = splitline[1].parse::<usize>().unwrap();
    let howmany: Range<usize> = 0..howmanyint;
    let fromcol = stacks[&cfrom][howmany].to_vec();
    let mut fromcolrev = Vec::<char>::new() ;
    // this line for challenge 1
    // for character in fromcol.iter().rev(){
    for character in fromcol.iter(){
        fromcolrev.push(*character);
    }
    let mut tocolumn = stacks.get_mut(&cto).unwrap();
    // println!("{:?}, {:?}",tocolumn,&fromcol);
    *tocolumn = [fromcol, tocolumn.to_vec()].concat();

}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}