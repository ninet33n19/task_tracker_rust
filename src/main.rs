use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

const TASKS_FILE: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        let tasks = read_from_json(TASKS_FILE);
        let next_id = if let Some(last_task) = tasks.last() {
            last_task.id + 1
        } else {
            1
        };
        println!("Initializing TaskManager with next ID: {}", next_id);
        TaskManager { tasks, next_id }
    }

    fn add_task(&mut self, description: String) {
        let now = Utc::now();
        let task = Task {
            id: self.next_id,
            description,
            status: Status::Pending,
            created_at: now,
            updated_at: now,
        };
        println!(
            "Task added: ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
            task.id, task.description, task.status, task.created_at, task.updated_at
        );
        self.tasks.push(task);
        self.next_id += 1;

        write_to_json(&self.tasks, TASKS_FILE);
    }

    fn update_description(&mut self, id: u32, new_description: String) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = new_description;
            task.updated_at = Utc::now();
            println!(
                "Task with ID {} updated. New description: {}. Updated At: {}",
                task.id, task.description, task.updated_at
            );
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn mark_in_progress(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = Status::InProgress;
            task.updated_at = Utc::now();
            println!(
                "Task with ID {} updated. Status: {:?}. Updated At: {}",
                task.id, task.status, task.updated_at
            );
        }
    }

    fn mark_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.status = Status::Completed;
            task.updated_at = Utc::now();
            println!(
                "Task with ID {} updated. Status: {:?}. Updated At: {}",
                task.id, task.status, task.updated_at
            );
        }
    }

    fn delete_task(&mut self, id: u32) {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            write_to_json(&self.tasks, TASKS_FILE);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn list_tasks(&mut self, status_arg: Option<&str>) {
        match status_arg {
            Some("pending") => {
                for task in &self.tasks {
                    if let Status::Pending = task.status {
                        println!(
                            "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
                            task.id,
                            task.description,
                            task.status,
                            task.created_at,
                            task.updated_at
                        );
                    }
                }
            }
            Some("inprogress") => {
                for task in &self.tasks {
                    if let Status::InProgress = task.status {
                        println!(
                            "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
                            task.id,
                            task.description,
                            task.status,
                            task.created_at,
                            task.updated_at
                        );
                    }
                }
            }
            Some("completed") => {
                for task in &self.tasks {
                    if let Status::Completed = task.status {
                        println!(
                            "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
                            task.id,
                            task.description,
                            task.status,
                            task.created_at,
                            task.updated_at
                        );
                    }
                }
            }
            Some(other) => {
                println!("Unknown status filter: {}", other);
            }
            None => {
                for task in &self.tasks {
                    println!(
                        "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
                        task.id, task.description, task.status, task.created_at, task.updated_at
                    );
                }
            }
        }
        write_to_json(&self.tasks, TASKS_FILE);
    }
}
// THIS FUNCTION WONT WORK BECAUSE WE CANNOT MUTATE A BORROWED VALUE
// fn delete_task(task_list: &mut Vec<Task>, id: u32) {
//     let mut index = 0;

//     for task in task_list {
//         if task.id != id {
//             index += 1;
//         } else {
//             task_list.remove(index);
//             println!("Task with ID {} has been deleted.", id);
//             return;
//         }
//     }
// }

fn write_to_json(task_list: &Vec<Task>, file_path: &str) {
    let json_data = serde_json::to_string(task_list).unwrap();
    println!("Writing tasks to {}: {}", file_path, json_data);
    std::fs::write(file_path, json_data).expect("Unable to write file");
}

fn read_from_json(file_path: &str) -> Vec<Task> {
    match std::fs::read_to_string(file_path) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn main() {
    let mut manager = TaskManager::new();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command> [args...]", args[0]);
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: {} add <description>", args[0]);
                return;
            }
            let description = args[2..].join(" ");
            manager.add_task(description);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: {} delete <id>", args[0]);
                return;
            }
            let id = args[2].parse().expect("I don't know man");
            manager.delete_task(id);
        }
        "list" => {
            let status_arg = args.get(2).map(|s| s.as_str());
            manager.list_tasks(status_arg);
        }
        "update" => {
            if args.len() < 3 {
                print!("Usage: {} update <id> <description>", args[0]);
                return;
            }
            let id = args[2].parse().expect("I don't know man");
            let description = args[3..].join(" ");
            manager.update_description(id, description);
        }
        "mark-in-progress" => {
            if args.len() < 3 {
                print!("Usage: {} mark-in-progress <id>", args[0]);
                return;
            }
            let id = args[2].parse().expect("I don't know man");
            manager.mark_in_progress(id);
        }
        "mark-completed" => {
            if args.len() < 3 {
                print!("Usage: {} mark-completed <id>", args[0]);
                return;
            }
            let id = args[2].parse().expect("I don't know man");
            manager.mark_completed(id);
        }
        _ => println!("Unknown command: {}", command),
    }
}
