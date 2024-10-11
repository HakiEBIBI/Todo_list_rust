use std::fs;
use std::fs::read_to_string;
use std::io;

fn main() -> std::io::Result<()> {
    let mut todo = String::new();
    println!("please give me your todo list : ");
    io::stdin()
        .read_line(&mut todo)
        .expect("can't read your line");

    let todo = todo.trim();
    //Open file.
    let mut todos: Vec<String> = match read_to_string("todo_list.txt") {
        Err(_) => Vec::new(),
        Ok(todo_list) => todo_list.lines().map(String::from).collect(),
    };

    if todo.contains("--delete") {
        let test = todo.split(" ").last();
        let line_number: usize = test.expect("Err").parse().unwrap();

        todos.remove(line_number - 1);
    } else {
        todos.push(todo.to_string());
    }

    fs::write("todo_list.txt", todos.join("\n")).expect("can't write to file!");

    Ok(())
}
