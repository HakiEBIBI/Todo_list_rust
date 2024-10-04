use std::io::{self};
use std::fs::File;
use std::io::prelude;
use std::io::Write;
fn main() {
    let mut file = File::create("todo-list.txt")
        .expect("could not create file!");
    let mut input = String::new();
    println!("Create your to do list");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    file.write(input.as_bytes()).expect("we cannot create a file for your todo list");

    println!("i created a file with the name of todo-list");
}
