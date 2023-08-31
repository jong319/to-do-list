#[derive(Debug)]
pub struct Todo {
    task: String,
    done: bool,
}


pub fn add(task: String) -> String{
        let result = Todo {
            task,
            done: false,
        };
        format!("{} {}", result.done, result.task)
    }
