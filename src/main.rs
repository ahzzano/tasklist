use serde::Deserialize;
use serde::Serialize;
use std;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::os::windows::prelude::FileExt;

/*
 * tasklist add               - prompts the user to add a task
 * tasklist list              - shows all tasks
 * tasklist resolve <task_id> - completes a task
 * tasklist clear             - clears the task list
 */

struct CLI {
    command: String,
    arg1: String,
    arg2: String,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    content: String,
    resolved: bool,
}

#[derive(Serialize, Deserialize)]
struct Data {
    tasks: Vec<Task>,
}

fn read_console_line(prompt: String) -> String {
    let mut line = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim_end().to_string()
}

fn add_task(data: &mut Data) {
    let last_task = data.tasks.last();

    let id = match last_task {
        Some(task) => task.id + 1,
        None => 0,
    };

    let task = Task {
        id,
        content: read_console_line(String::from("Task> ")),
        resolved: false,
    };

    data.tasks.push(task);
}

fn list_tasks(data: &Data) {
    for i in &data.tasks {
        if i.resolved {
            println!("[x] {} - {}", i.id, i.content);
        } else {
            println!("[ ] {} - {}", i.id, i.content);
        }
    }
}

fn clear_tasks(data: &mut Data) {
    data.tasks.clear();
}

fn get_cli_args() -> CLI {
    let command = std::env::args().nth(1).expect("no command given");
    let arg1 = match std::env::args().nth(2) {
        None => String::from(""),
        Some(s) => s.to_string(),
    };

    let arg2 = match std::env::args().nth(3) {
        None => String::from(""),
        Some(s) => s.to_string(),
    };

    CLI {
        command,
        arg1,
        arg2,
    }
}

fn resolve_task(data: &mut Data, cli: &CLI) {
    if cli.arg1 == "" {
        return;
    }

    let task_id = cli.arg1.parse::<i32>().unwrap();

    for i in &mut data.tasks {
        if i.id == task_id {
            i.resolved = true;
            break;
        }
    }
}

fn main() {
    let cli_data = get_cli_args();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.json")
        .expect("unable to create/read file");

    let mut json_string = String::new();

    file.read_to_string(&mut json_string)
        .expect("Error reading file");

    let dat_result = serde_json::from_str(json_string.as_str());

    let mut data = match dat_result {
        Ok(file) => file,
        Err(_) => Data { tasks: Vec::new() },
    };

    match cli_data.command.as_str() {
        "list" => list_tasks(&data),
        "add" => add_task(&mut data),
        "resolve" => resolve_task(&mut data, &cli_data),
        "clear" => clear_tasks(&mut data),
        _ => println!("Invalid command"),
    };

    file.set_len(0).expect("Unable to truncate");

    let t_s = serde_json::to_string(&data).expect("unable to parse task");

    file.set_len(1).expect("Unable to truncate");
    file.seek_write(t_s.as_bytes(), 0)
        .expect("Failed to write to file");
}
