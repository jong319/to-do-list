#[derive(Debug)]
pub struct Todo {
    task: String,
    done: bool,
}

pub fn add(task: String) -> String {
    let result = Todo { task, done: false };
    format!("{} {}\n", result.done, result.task)
}
