use serde::Deserialize;
use serde::Serialize;
use std;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::os::windows::prelude::FileExt;

/*
 * tasklist add               - prompts the user to add a task
 * tasklist print             - shows all tasks
 * tasklist resolve <task_id> - completes a task
 * tasklist clear             - clears the task list
 */

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    content: String,
    resolved: bool,
}

fn read_console_line(prompt: String) -> String {
    let mut line = String::new();

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    line.trim_end().to_string()
}

#[derive(Serialize, Deserialize)]
struct Data {
    tasks: Vec<Task>,
}

fn add_task(data: &mut Data) {
    let task = Task {
        id: 23,
        content: read_console_line(String::from("Task> ")),
        resolved: false,
    };

    data.tasks.push(task);
}

fn main() {
    let command = std::env::args().nth(1).expect("no command given");

    println!("Command: {}", command);
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
    let mut dat: Data;

    match dat_result {
        Ok(file) => dat = file,
        Err(error) => dat = Data { tasks: Vec::new() },
    }

    file.set_len(0).expect("Unable to truncate");

    let t = Task {
        id: 1,
        content: String::from("Hello"),
        resolved: false,
    };

    add_task(&mut dat);

    let t_s = serde_json::to_string(&dat).expect("unable to parse task");

    file.set_len(1).expect("Unable to truncate");
    file.seek_write(t_s.as_bytes(), 0)
        .expect("Failed to write to file");
}
