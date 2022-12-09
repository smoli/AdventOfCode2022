use std::fs;
use std::str::FromStr;

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

///
/// Read input and convert it to a 2d grid using a nested Vec<Vec<T>
///
/// Uses the initial value mainly to determine the type.
///
/// Input is expected to be full.
///
pub fn make_grid<T: Clone + FromStr>(input: Vec<String>, initial: T) -> Vec<Vec<T>> {
    //
    // input.into_iter()
    //     .map(|line| line.split("\r\n").map(|c| c.parse::<T>().unwrap()).collect())
    //     .collect()


    let w = input.get(0).unwrap().len();
    let h = input.len();



    let mut map:Vec<Vec<T>> = vec![vec![initial; w]; h];

    let mut y = 0;
    for l in input {
        let mut x = 0;
        for c in l.split("") {
            match c.parse::<T>() {
                Err(_) => {},
                Ok(c) => {
                    map[y][x] = c;
                    x += 1;
                }
            };
        }

        y += 1;
    }

    map
}

