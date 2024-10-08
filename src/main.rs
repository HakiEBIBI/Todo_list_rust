use std::fs::OpenOptions;
use std::io::{self, Write};
fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo-list.txt")
        .expect("unable to open the file");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let input = input.trim();

    println!("i created a file with the name of todo-list");
    writeln!(file, "{}", input).expect("err");
}
