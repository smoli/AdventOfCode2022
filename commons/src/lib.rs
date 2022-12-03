use std::fs;

pub fn read_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    let lines = contents.split("\r\n");

    let mut r: Vec<String> = vec![];
    for line in lines {
        r.push(String::from(line));
    }

    return r;
}

