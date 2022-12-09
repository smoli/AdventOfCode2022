use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::Direction::{Down, Error, Left, Right, Up};

enum Direction {
    Right,
    Left,
    Up,
    Down,
    Error,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            Left => write!(f, "Left"),
            Right => write!(f, "Right"),
            Up => write!(f, "Up"),
            Down => write!(f, "Down"),
            Error => write!(f, "Error"),
        }
    }
}

struct Motion {
    direction: Direction,
    steps: u8,
}

impl Display for Motion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.steps)
    }
}


fn follow(input: &Vec<Motion>) -> HashMap<String, bool> {
    let mut positions: HashMap<String, bool> = HashMap::new();

    let mut hx: i32 = 0;
    let mut hy: i32 = 0;
    let mut tx: i32 = 0;
    let mut ty: i32 = 0;

    positions.insert(format!("{tx}|{ty}"), true);

    for i in input {
        for _ in 0..i.steps {
            match i.direction {
                Right => hx += 1,
                Left => hx -= 1,
                Up => hy -= 1,
                Down => hy += 1,
                Error => return positions
            }

            if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                tx += (hx - tx).signum();
                ty += (hy - ty).signum();
            }

            positions.insert(format!("{tx}|{ty}"), true);
        }
    }

    positions
}

fn parse(input: &Vec<String>) -> Vec<Motion> {
    let mut result: Vec<Motion> = vec![];
    let re: Regex = Regex::new(r"^(\w)\s(\d+)").unwrap();


    for l in input {
        let c = re.captures(l).unwrap();

        result.push(Motion {
            direction: match c.get(1) {
                None => Error,
                Some(c) => match c.as_str() {
                    "R" => Right,
                    "L" => Left,
                    "U" => Up,
                    "D" => Down,
                    _ => Error
                }
            },

            steps: c.get(2).unwrap().as_str().parse().unwrap(),
        })
    }

    result
}


fn main() {
    let data = commons::read_input("input.txt");

    let input = parse(&data);

    let res = follow(&input);

    println!("Part One: {}", res.values().len());
}
