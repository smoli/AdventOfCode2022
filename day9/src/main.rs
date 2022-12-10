use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
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


struct Position {
    x: i32,
    y: i32,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position { x: self.x, y: self.y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}


fn follow_step(hx: i32, hy: i32, tx: i32, ty: i32) -> (i32, i32) {
    return if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
        (tx + (hx - tx).signum(),
         ty + (hy - ty).signum())
    } else {
        (tx, ty)
    };
}

fn follow(input: &Vec<Motion>, rope_length: usize) -> HashMap<String, bool> {
    let mut positions: HashMap<String, bool> = HashMap::new();
    let mut rope: Vec<Position> = vec![Position { x: 0, y: 0 }; rope_length];

    let tail = rope.last().unwrap();
    positions.insert(format!("{}|{}", tail.x, tail.x), true);

    for i in input {
        for _ in 0..i.steps {
            let head = rope.get(0).unwrap();
            let mut hx = head.x;
            let mut hy = head.y;

            match i.direction {
                Right => hx += 1,
                Left => hx -= 1,
                Up => hy -= 1,
                Down => hy += 1,
                Error => {}
            }

            rope[0] = Position { x: hx, y: hy };

            for k in 1..rope_length {
                let head = rope.get(k - 1).unwrap();
                let tail = rope.get(k).unwrap();
                let (tx, ty) = follow_step(head.x, head.y, tail.x, tail.y);

                rope[k] = Position { x: tx, y: ty };
            }

            let tail = rope.last().unwrap();
            positions.insert(format!("{}|{}", tail.x, tail.y), true);
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

    let res = follow(&input, 2);
    println!("Part One: {}", res.values().len());

    let res2 = follow(&input, 10);
    println!("Part Two: {}", res2.values().len());
}
