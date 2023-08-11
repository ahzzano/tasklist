use serde::Deserialize;
use serde::Serialize;
use std;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::os::windows::prelude::FileExt;

/*
 * tasklist add <project|task>                   - prompts the user to add a task
 * tasklist remove <project|task> <task_id|project_tag>
 * tasklist list <all|project|active>               - shows all tasks
 * tasklist resolve <task_id>                    - completes a task
 * tasklist clear              - clears the task list
 *
 * Tasks are organized by Project and Groups
 */

struct CLI {
    command: String,
    arg1: String,
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: i32,
    content: String,
    resolved: bool,
    project: String,
    group: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    tasks: Vec<Task>,
    groups: Vec<String>,
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    tag: String,
    description: String,
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
    let task_name = read_console_line(String::from("Task> "));
    let task_project = read_console_line(String::from("Project> "));
    let task_group = read_console_line(String::from("Tag> "));

    if !data.groups.contains(&task_group) {
        data.groups.push(task_group.clone());
    }

    let task = Task {
        id,
        content: task_name,
        project: task_project,
        group: task_group,
        resolved: false,
    };

    data.tasks.push(task);
}

fn print_all_tasks_in_vec(vec: Vec<&Task>) {
    if vec.len() <= 0 {
        return;
    }
    let last_elem = vec.last().unwrap();

    for task in &vec {
        if task.id == last_elem.id {
            print!("└");
        } else {
            print!("├");
        }

        if task.resolved {
            print!("[x]")
        } else {
            print!("[ ]")
        }

        print!("- {:<3} - {:<10} ", task.id, task.content);
        if task.project != String::from("") {
            print!("- {:<10}", task.project)
        }
        print!("\n");
    }
}

fn list_tasks(data: &Data) {
    for i in &data.groups {
        println!("{ }", i);

        let a: Vec<&Task> = data.tasks.iter().filter(|&x| &x.group == i).collect();

        print_all_tasks_in_vec(a);

        print!("\n");
    }
    println!("No Groupings");

    let non_grouped_tasks: Vec<&Task> = data.tasks.iter().filter(|&x| x.group.is_empty()).collect();

    print_all_tasks_in_vec(non_grouped_tasks);
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

    CLI { command, arg1 }
}

fn add_project(data: &mut Data) {
    let project_name = read_console_line(String::from("Project> "));
    let project_tag = read_console_line(String::from("Tag> "));
    let project_description = read_console_line(String::from("Description> "));

    let project = Project {
        name: project_name,
        tag: project_tag,
        description: project_description,
    };

    data.projects.push(project);
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
        Err(_) => Data {
            tasks: Vec::new(),
            groups: Vec::new(),
            projects: Vec::new(),
        },
    };

    match cli_data.command.as_str() {
        "list" => list_tasks(&data),
        "add" => {
            if cli_data.arg1.as_str() == "task" {
                add_task(&mut data);
            }
            if cli_data.arg1.as_str() == "project" {
                add_project(&mut data);
            }
        }
        "resolve" => resolve_task(&mut data, &cli_data),
        "clear" => clear_tasks(&mut data),
        _ => println!("Invalid command"),
    };

    file.set_len(0).expect("Unable to truncate");

    let t_s = serde_json::to_string_pretty(&data).expect("unable to parse task");

    file.set_len(1).expect("Unable to truncate");
    file.seek_write(t_s.as_bytes(), 0)
        .expect("Failed to write to file");
}
