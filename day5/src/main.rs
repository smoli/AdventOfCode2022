use regex::Regex;

fn get_initial_state(data: &Vec<String>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];
    let re = Regex::new(r"\s*\[").unwrap();

    for line in data {
        if !re.is_match(line)
        {
            break;
        }
        for (i, c) in line.chars().enumerate() {
            if i > 0 && (i - 1) % 4 == 0 && c != ' ' {
                let col = (i - 1) / 4;
                while result.len() <= col {
                    result.push(vec![]);
                }
                result[col].insert(0, c);
            }
        }
    }

    result
}

struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

fn get_commands(data: &Vec<String>) -> Vec<Move> {
    let mut result: Vec<Move> = vec![];
    let re = Regex::new(r"^move\s*(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();

    for line in data {
        let caps = re.captures(line);

        match caps {
            None => {}

            Some(captures) => {
                result.push(Move {
                    amount: captures.get(1).map_or(1, |m| m.as_str().parse().unwrap()),
                    from: captures.get(2).map_or(1, |m| m.as_str().parse().unwrap()) - 1,
                    to: captures.get(3).map_or(1, |m| m.as_str().parse().unwrap()) - 1,
                })
            }
        }
    }

    result
}

fn part_one() {

    let data = commons::read_input("input.txt");
    let mut state = get_initial_state(&data);
    let moves = get_commands(&data);


    for m in moves {
        for _ in 0..m.amount {
            let v = state[m.from].pop().unwrap();
            state[m.to].push(v);
        }
    }

    for col in &state[0..] {
        match col.last() {
            None => print!("_"),

            Some(c) => print!("{c}")
        }
    }

    print!("\n");
}

fn part_two() {
    let data = commons::read_input("input.txt");
    let mut state = get_initial_state(&data);
    let moves = get_commands(&data);


    for m in moves {
        let mut buffer: Vec<char> = vec![];
        for _ in 0..m.amount {
            let v = state[m.from].pop().unwrap();
            buffer.push(v);
        }
        for _ in 0..m.amount {
            let v = buffer.pop().unwrap();
            state[m.to].push(v);
        }
    }

    for col in &state[0..] {
        match col.last() {
            None => print!("_"),

            Some(c) => print!("{c}")
        }
    }

    print!("\n");
}


fn main() {
    part_one();
    part_two();
}
