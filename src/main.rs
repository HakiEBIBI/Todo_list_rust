use std::fs;
use std::fs::read_to_string;
// use std::fs::read_to_string;
use std::io;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// struct Todo{
//     content: String,
// }

// fn main() -> std::io::Result<()> {
//     let mut todo = String::new();
//      println!("please give me your todo list : ");
//      io::stdin()
//          .read_line(&mut todo)
//          .expect("can't read your line");

//      let todo = todo.trim();
//       let mut todos: Vec<Todo> = match read_to_string("todo_list.txt") {
//           Err(_) => Vec::new(),
//           Ok(todo_list) => todo_list.lines().map(String::from).collect(),
//       };

//      if user_input.contains("--delete") {
//          let test = user_input.split(" ").last();
//          let line_number: usize = test.expect("Err").parse().expect("i cant execute the --delete");

//         todos.remove(line_number - 1);
//      } else {
//         todos.push(user_input.to_string());
//}

//       let mut todos : Vec<Todo> = Vec::new();
//     todos.push(todo);

//    //write

//    fs::write("todos.json", serde_json::to_string(&mut todos).expect("err")).expect("can't write");
//      // fs::write("text.txt", todos.join("\n")).expect("can't write");

//     Ok(())
// }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    content: String,
}

fn main() {
    println!("please write a todo");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("can't read your line");

    let user_input = user_input.trim();

    let mut todos: Vec<Todo> = match read_to_string("todos.json") {
        Err(_) => Vec::new(),
        Ok(todos) => serde_json::from_str(&todos).expect("Cannot open the file"),
    };

    if user_input.contains("--delete") {
        let test = user_input.split(" ").last();
        let line_number: usize = test
            .expect("Err")
            .parse()
            .expect("i cant execute the --delete");

        todos.remove(line_number - 1);
    } else {
        todos.push(Todo {
            content: user_input.to_string(),
        });
    }

    fs::write("todos.json", serde_json::to_string(&todos).expect("err"))
        .expect("can't write on the file");
}
