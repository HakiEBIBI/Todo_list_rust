# TODO README
This Rust application manages a simple to-do list using JSON for data storage. Users can add, delete, mark tasks as done or undone, set due dates, and list all tasks.
## Installation of Rust

Click on the link below:

[Installation Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)

Follow the instructions provided there. After that, we will clone the code from [GitHub](https://github.com/HakiEBIBI/todo_list_rust).

Go to **<> Code** and click on **SSH**. Copy the link and paste it into your command prompt.

The installation procedure is complete.

## Simple To-Do List Manager in Rust
### Key Components

### Dependencies
- **chrono**: For date handling.
- **clap**: For command-line argument parsing.
- **serde** and **serde_json**: For JSON serialization/deserialization.

### Main Functionality
1. **Reading Tasks**: Loads tasks from `todo.json`, initializing an empty list if the file doesn't exist.
2. **Command Handling**:
   - Delete a task with `--delete <index>`.
   - Mark a task done with `--done <index>`.
   - Mark a task undone with `--undone <index>`.
   - Set a due date with `--due <date> --id <number>`.
   - List tasks with `--list`.
   - Add a new task by running without flags and entering the description.

3. **Writing Tasks**: Saves the updated list back to `todo.json`.
