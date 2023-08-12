use serde::Deserialize;
use serde::Serialize;

pub struct CLI {
    pub command: String,
    pub arg1: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub resolved: bool,
    pub project: String,
    pub group: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub tasks: Vec<Task>,
    pub groups: Vec<String>,
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub tag: String,
    pub description: String,
}
