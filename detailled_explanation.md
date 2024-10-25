# Example Usage of Simple To-Do List Manager in Rust

In this example, we will demonstrate how to use the Simple To-Do List Manager application through the command line by performing a series of operations: adding a new task, listing all tasks, marking a task as done, deleting a task, and setting a due date for a task.

1. **Adding a New Task**: Run the application without any flags to add a new task:

    ```bash
    cargo run
    ```

    **Output:**
    ```
    Enter a to-do list
    ```

    **User Input:**
    ```
    Buy groceries
    ```

2. **Listing All Tasks**: To view all tasks, use the `--list` flag:

    ```bash
    cargo run -- --list
    ```

    **Output:**
    ```
    1. Buy groceries, Undone
    ```

3. **Marking the Task as Done**: To mark the first task as done, use the `--done` flag:

    ```bash
    cargo run -- --done 1
    ```

4. **Listing Tasks Again**: List tasks again to confirm the status change:

    ```bash
    cargo run -- --list
    ```

    **Output:**
    ```
    1. Buy groceries, Done
    ```

5. **Adding Another Task**: Add another task by running the application again:

    ```bash
    cargo run
    ```

    **Output:**
    ```
    Enter a to-do list
    ```

    **User Input:**
    ```
    Call the doctor
    ```

6. **Deleting a Task**: To delete the first task, use the `--delete` flag:

    ```bash
    cargo run -- --delete 1
    ```

7. **Listing Tasks After Deletion**: List tasks again to confirm deletion:

    ```bash
    cargo run -- --list
    ```

    **Output:**
    ```
    1. Call the doctor, Undone
    ```

8. **Setting a Due Date for the Remaining Task**: Set a due date for the remaining task using the `--due` and `--id` flags:

    ```bash
    cargo run -- --due 2024-12-31 --id 1
    ```

9. **Final Listing of Tasks with Due Dates**: Finally, list tasks to see the due date:

    ```bash
    cargo run -- --list
    ```

    **Output:**
    ```
    1. Call the doctor, Undone, Due: 2024-12-31
    ```

This example illustrates how to interact with the Simple To-Do List Manager through various command-line operations, including adding tasks, listing them, marking them as done, deleting them, and setting due dates effectively.