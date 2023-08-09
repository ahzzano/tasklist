use serde::Deserialize;
use serde::Serialize;
use std;
use std::fs::File;
use std::io::Write;

/*
 * tasklist add               - prompts the user to add a task
 * tasklist print             - shows all tasks
 * tasklist resolve <task_id> - completes a task
 */

#[derive(Serialize, Deserialize)]
struct Task {
    content: String,
    resolved: bool,
}

fn main() {
    let command = std::env::args().nth(1).expect("no command given");
    println!("Command: {}", command);

    let t = Task {
        content: String::from("Hello"),
        resolved: false,
    };

    let mut file = File::create("data.json").expect("creation failed");
    let t_s = serde_json::to_string(&t).expect("unable to parse task");

    file.write_all(t_s.as_bytes())
        .expect("Failed to write to file");
}
