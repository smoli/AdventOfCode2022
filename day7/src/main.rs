use std::collections::HashMap;
use regex::Regex;

fn parse_console(input: Vec<String>) -> HashMap<i32, i64> {
    let cd = Regex::new(r"^\$ cd (.+)").unwrap();
    let file_entry = Regex::new(r"(\d+)\s(.+)").unwrap();

    let mut path: Vec<i32> = vec![];
    let mut sizes: HashMap<i32, i64> = HashMap::new();
    let mut folder_id: i32 = 0;

    for l in input {
        match cd.captures(&l.as_str()) {
            None => {}
            Some(captures) => {
                if captures.get(1).unwrap().as_str() == ".." {
                    path.pop();
                } else {
                    path.push(folder_id);
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
                        None => sizes.insert(*dir, size),
                        Some(v) => sizes.insert(*dir, size + v),
                    };
                }
            }
        }
    }

    return sizes;
}


fn main() {
    let data = commons::read_input("input.txt");

    let sizes = parse_console(data);

    let mut sum: i64 = 0;
    for v in sizes.values() {
        if *v <= 100000 {
            sum += *v;
        }
    }

    // Part One
    println!("{sum}");

    // Root has id 0
    let used = sizes.get(&0i32).unwrap();
    let avail = 70000000;
    let needed = 30000000;

    let mut candidates: Vec<i64> = vec![];

    for v in sizes.values() {

        if (avail - used + v) >= needed {
            candidates.push(*v);
        }
    }

    println!("{}", candidates.iter().min().unwrap());
}
