use std::{fs::File, io::{BufReader, BufRead}};

const TODO: &str = "\u{2610}";
const DONE: &str = "\u{2611}";


pub fn list(reader: &mut BufReader<File>) -> String{
    let mut lines = String::new();
    let mut result = String::new();
    let mut index = 0;
    
    while reader.read_line(&mut lines).unwrap() > 0 {
        index += 1;

        if lines.starts_with("true") {
            let lines = lines.trim_start_matches("true ");
            let task = format!("{} {}: {}", DONE,index,lines);
            result.push_str(task.as_str())
        }
        else {
            let lines = lines.trim_start_matches("false ");
            let task = format!("{} {}: {}", TODO,index,lines);
            result.push_str(task.as_str())
        }
        lines.clear();
    };
    
    if index == 0 {
        println!("To Do list is empty!")
    }
    result
    
}