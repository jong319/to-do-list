use std::{fs::File, io::{BufReader, BufRead}};


pub fn update(reader: &mut BufReader<File>, index: String) -> (String, String) {
    //convert numbered string to integer
    let mut index = index.parse::<u32>().expect("not a valid index, please put a number");
    index -= 1;
    let mut result = String::new();

    let mut count = 0;
    let mut l = String::new();
    let mut updated_task = String::new();
    while reader.read_line(&mut l).unwrap() > 0 {
        if count == index {
            if l.starts_with("true") {
                let update = l.replacen("true", "false", 1);
                result.push_str(update.as_str());
                updated_task.push_str(update.replacen("false", "Did not complete:", 1).as_str())
            }

            else if l.starts_with("false") {
                let update = l.replacen("false", "true", 1);
                result.push_str(update.as_str());
                updated_task.push_str(update.replacen("true", "Completed:", 1).as_str())
            }
        }

        else {
            result.push_str(l.as_str())
        }

        count += 1;
        l.clear();
    }

    (result, updated_task)
    
}
