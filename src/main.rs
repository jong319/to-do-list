use clap::{Command, arg};
use std::{fs::OpenOptions, io::{BufReader, Write}};
mod list; 
mod add;
mod update;

const FILE_NAME: &str = ".todo.txt";
const LIST: &str = "list";
const ADD: &str = "add";
const UPDATE: &str = "update";
const CLEAR: &str = "clear";


fn main() {
    let file_path = file_path();
    let open_file_config = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&file_path)
        .unwrap();
    let mut reader = BufReader::new(open_file_config);
    match build().get_matches().subcommand().unwrap() {
        (LIST, _) => {
            let results = list::list(&mut reader);
            print!("{}",results)
        },
        (ADD, task) => {
            let results = add::add(task.get_one::<String>("TASK").unwrap().to_string());
            let mut writer = OpenOptions::new().create(true).append(true).open(file_path).unwrap();
            writer.write_all(results.as_bytes()).unwrap();
            print!("{} successfully added\n", task.get_one::<String>("TASK").unwrap().as_str());
        }
        (UPDATE, index) => {
            let task = index.get_one::<String>("INDEX").expect("not an integer!").to_owned();
            let result = update::update(&mut reader, task);
            let mut writer = OpenOptions::new().write(true).open(file_path).unwrap();
            //clear entire file and rewrite
            writer.set_len(0).unwrap();
            writer.write_all(result.0.as_bytes()).unwrap();
            print!("{}", result.1)
        },
        (CLEAR, _) => {
            let writer = OpenOptions::new().write(true).open(file_path).unwrap();
            writer.set_len(0).unwrap();
        }
        _ => unreachable!(),
    }


}



fn build() -> Command {
    Command::new("To do list")
    .version("1.0.0")
    .about("Simple CLI To do list")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
        Command::new(LIST).about("lists all tasks")
    )
    .subcommand(
        Command::new(ADD).about("add new task")
        .arg(arg!(<TASK>).required(true))
    )
    .subcommand(Command::new(UPDATE).about("update progress of task")
        .arg(arg!(<INDEX>).required(true))
    )
    .subcommand(Command::new(CLEAR).about("clears the entire list"))
}

fn file_path() -> String {
    format!("./{}",FILE_NAME)
}
