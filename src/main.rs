use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string};
use std::io::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
    completed: bool,
}

#[derive(Parser, Debug)]
#[command(version = "0.1")]
struct Flag {
    #[arg(long, short)]
    delete: Option<usize>,
    #[arg(long)]
    done: Option<usize>,
}

fn main() -> std::io::Result<()> {
    let flags = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todo.json") {
        Ok(file_content) => match serde_json::from_str(&file_content) {
            Ok(todos) => todos,
            Err(e) => {
                eprintln!("Error deserializing JSON: {}", e);
                Vec::new()
            }
        },
        Err(_) => Vec::new(),
    };

    if let Some(number_line) = flags.delete {
        if number_line > 0 && number_line <= todos.len() {
            todos.remove(number_line - 1);
        }
    } else if let Some(number_line) = flags.done {
        if number_line > 0 && number_line <= todos.len() {
            todos[number_line - 1].completed = true;
        }
    } else {
        let mut user_input = String::new();
        println!("Enter a to-do list item:");
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();
        if !user_input.is_empty() {
            todos.push(Todo {
                content: user_input.to_string(),
                completed: false,
            });
        }
    }

    fs::write(
        "todo.json",
        serde_json::to_string(&todos).expect("Error serializing"),
    )
    .expect("Can't write");

    Ok(())
}
