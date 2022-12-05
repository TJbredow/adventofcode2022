use std::fs;
fn main() {
    let file_path = String::from("./input");
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let sacks = contents.lines();
    for num in (3..5){
        println!("{}", (4..6).contains(&num));
        print_type_of(&num);
    }
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}