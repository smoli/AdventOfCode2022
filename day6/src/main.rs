
fn find_marker(input: String, len: usize) -> i32 {
    let mut window: Vec<char> = vec![];

    for (i, d) in input.chars().enumerate() {
        let p = window.iter().position(|&c| c == d);
        match p {
            None => if window.len() < len - 1 {
                        window.push(d);
                    } else {
                        return i as i32 + 1;
                    },
            Some(i) => {
                window.reverse();
                window.truncate(window.len() - (i + 1));
                window.reverse();
                window.push(d);
            }
        }
    }

    return -1;
}

fn process(len: usize) {
    let data = commons::read_input("input.txt");

    for l in data {
        println!("{}", find_marker(l, len));
    }

}

fn main() {
    process(4);
    process(14);
}
