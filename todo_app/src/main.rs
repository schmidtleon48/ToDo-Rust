use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct ToDoList {
    tasks: Vec<Task>,
}

impl ToDoList {
    fn new() -> ToDoList {
        ToDoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn complete_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks[index].complete();
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!(
                "{}: {} [{}]",
                i,
                task.description,
                if task.completed { "completed" } else { "not completed" }
            );
        }
    }
}

fn main() {
    let todo_list = Arc::new(Mutex::new(ToDoList::new()));
    let todo_list_clone = Arc::clone(&todo_list);

    // Simulating multithreading (tasks could be managed by different threads)
    let handler = thread::spawn(move || {
        let stdin = io::stdin();
        loop {
            println!("1. Add task");
            println!("2. Complete task");
            println!("3. List tasks");
            println!("4. Exit");

            print!("Enter your choice: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");

            let choice: u32 = input.trim().parse().unwrap_or(0);

            let mut todo_list = todo_list_clone.lock().unwrap();

            match choice {
                1 => {
                    print!("Enter task description: ");
                    io::stdout().flush().unwrap();
                    let mut description = String::new();
                    stdin.read_line(&mut description).expect("Failed to read line");
                    let task = Task::new(description.trim().to_string());
                    todo_list.add_task(task);
                }
                2 => {
                    print!("Enter task number to complete: ");
                    io::stdout().flush().unwrap();
                    let mut index = String::new();
                    stdin.read_line(&mut index).expect("Failed to read line");
                    let index: usize = index.trim().parse().unwrap_or(0);

                    match todo_list.complete_task(index) {
                        Ok(_) => println!("Task completed"),
                        Err(err) => println!("Error: {}", err),
                    }
                }
                3 => {
                    todo_list.list_tasks();
                }
                4 => {
                    break;
                }
                _ => {
                    println!("Invalid choice");
                }
            }
        }
    });

    handler.join().unwrap();
}
