use std::collections::HashMap;
use regex::Regex;

fn parse_console(input: Vec<String>) -> HashMap<String, i64> {
    let cd = Regex::new(r"^\$ cd (.+)").unwrap();
    let file_entry = Regex::new(r"(\d+)\s(.+)").unwrap();

    let mut path: Vec<String> = vec![];
    let mut sizes: HashMap<String, i64> = HashMap::new();
    let mut folder_id: i32 = 0;

    for l in input {
        match cd.captures(&l.as_str()) {
            None => {}
            Some(captures) => {
                if captures.get(1).unwrap().as_str() == ".." {
                    path.pop();
                } else {
                    path.push(String::from(folder_id.to_string()));
                    folder_id += 1;
                }
            }
        }


        match file_entry.captures(&l.as_str()) {
            None => {}
            Some(captures) => {
                let size: i64 = captures.get(1).unwrap().as_str().parse().unwrap();

                for dir in &path {
                    match sizes.get(dir) {
                        None => sizes.insert(String::from(dir), size),
                        Some(v) => sizes.insert(String::from(dir), size + v),
                    };
                }
            }
        }
    }

    return sizes;
}


fn main() {
    // let data = commons::read_input("sampleInput.txt");
    let data = commons::read_input("input.txt");

    let sizes = parse_console(data);

    let mut sum: i64 = 0;
    for v in sizes.values() {
        if *v <= 100000 {
            sum += *v;
        }
    }

    println!("{sum}")
}
