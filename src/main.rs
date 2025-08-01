use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

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

fn add_task(task_list: &mut Vec<Task>, id: u32, description: String) {
    let now = Utc::now();
    let task = Task {
        id,
        description,
        status: Status::Pending,
        created_at: now,
        updated_at: now,
    };

    println!(
        "Task added: ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
        task.id, task.description, task.status, task.created_at, task.updated_at
    );

    task_list.push(task);
    write_to_json(task_list, "tasks.json");
}

fn update_task_status(task_list: &mut Vec<Task>, id: u32, new_status: Status) {
    for task in task_list {
        if task.id == id {
            task.status = new_status;
            task.updated_at = Utc::now();
            println!(
                "Task with ID {} updated to status {:?} at {}",
                task.id, task.status, task.updated_at
            );
            return;
        }
    }
    println!("Task with ID {} not found.", id);
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

fn delete_task(task_list: &mut Vec<Task>, id: u32) {
    if let Some(pos) = task_list.iter().position(|task| task.id == id) {
        task_list.remove(pos);
        println!("Task with ID {} has been deleted.", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

fn list_tasks(tasks_file: &str) {
    let task_list_from_file = read_from_json(tasks_file);
    for task in task_list_from_file {
        println!(
            "ID: {}, Description: {}, Status: {:?}, Created At: {}, Updated At: {}",
            task.id, task.description, task.status, task.created_at, task.updated_at
        );
    }
}

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
    let mut task_list: Vec<Task> = Vec::new();
    const TASKS_FILE: &str = "tasks.json";

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <command> [args...]", args[0]);
        return;
    }
    let command = &args[1];
    match command.as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage: {} add <id> <description>", args[0]);
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            let description = args[3..].join(" ");
            add_task(&mut task_list, id, description);
        }
        "update" => {
            if args.len() < 4 {
                println!("Usage: {} update <id> <status>", args[0]);
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            let status = match args[3].as_str() {
                "Pending" => Status::Pending,
                "InProgress" => Status::InProgress,
                "Completed" => Status::Completed,
                _ => {
                    println!("Invalid status. Use Pending, InProgress, or Completed.");
                    return;
                }
            };
            update_task_status(&mut task_list, id, status);
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: {} delete <id>", args[0]);
                return;
            }
            let id: u32 = args[2].parse().unwrap_or(0);
            delete_task(&mut task_list, id);
        }
        "list" => {
            list_tasks(TASKS_FILE);
        }
        _ => println!("Unknown command: {}", command),
    }
}
