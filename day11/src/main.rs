use std::fmt::{Display, Formatter};
use regex::Regex;
use crate::Operation::{Multiply, Plus};

enum Operation {
    Multiply(i64),
    Plus(i64),
    Square,
}

impl Clone for Operation {
    fn clone(&self) -> Self {
        match self {
            Multiply(v) => Multiply(*v),
            Operation::Plus(v) => Plus(*v),
            Operation::Square => Operation::Square
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Multiply(v) => write!(f, "* {}", v),
            Plus(v) => write!(f, "+ {}", v),
            Operation::Square => write!(f, "* old"),
        }
    }
}

struct Monkey {
    num: usize,
    items: Vec<i64>,
    op: Operation,
    test: i64,
    when_true: i64,
    when_false: i64,
    inspections: usize,
}

impl Monkey {
    pub fn add_item(&mut self, item: i64) {
        self.items.push(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl Clone for Monkey {
    fn clone(&self) -> Self {
        return Monkey {
            num: self.num,
            items: self.items.clone(),
            op: self.op.clone(),
            test: self.test,
            when_true: self.when_true,
            when_false: self.when_false,
            inspections: self.inspections,
        };
    }
}


fn parse(input: &Vec<String>) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = vec![];
    let r_monkey = Regex::new(r"^Monkey (\d+):").unwrap();
    let r_items = Regex::new(r"^\s+Starting items: (.*)").unwrap();
    let r_operation = Regex::new(r"^\s+Operation: new = old (.) (\d+|old)").unwrap();
    let r_test = Regex::new(r"^\s+Test: divisible by (\d+)").unwrap();
    let r_true = Regex::new(r"^\s+If true: throw to monkey (\d+)").unwrap();
    let r_false = Regex::new(r"^\s+If false: throw to monkey (\d+)").unwrap();
    let mut i = 0;

    loop {
        if i >= input.len() {
            break;
        }
        let mut l = input.get(i).unwrap();

        let monkey_num = r_monkey.captures(l).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();

        i += 1;
        l = input.get(i).unwrap();
        let items = r_items.captures(l).unwrap()
            .get(1).unwrap().as_str()
            .split(", ").collect::<Vec<&str>>()
            .into_iter().map(|s| s.parse().unwrap()).collect();

        i += 1;
        l = input.get(i).unwrap();
        let opc = r_operation.captures(l).unwrap();
        let op = match opc.get(1).unwrap().as_str() {
            "*" => match opc.get(2).unwrap().as_str() {
                "old" => Operation::Square,

                _ => Operation::Multiply(opc.get(2).unwrap().as_str().parse::<i64>().unwrap() as i64)
            },

            "+" => Operation::Plus(opc.get(2).unwrap().as_str().parse::<i64>().unwrap() as i64),

            _ => panic!()
        };

        i += 1;
        l = input.get(i).unwrap();
        let test = r_test.captures(l).unwrap().get(1).unwrap().as_str().parse().unwrap();

        i += 1;
        l = input.get(i).unwrap();
        let when_true = r_true.captures(l).unwrap().get(1).unwrap().as_str().parse().unwrap();

        i += 1;
        l = input.get(i).unwrap();
        let when_false = r_false.captures(l).unwrap().get(1).unwrap().as_str().parse().unwrap();

        result.push(Monkey {
            num: monkey_num,
            items,
            op,
            test,
            when_true,
            when_false,
            inspections: 0,
        });

        i += 2;
    }

    result
}


fn perform_operation(op: &Operation, value: i64) -> i64 {
    return match op {
        Multiply(v) => value * v,
        Operation::Plus(v) => value + v,
        Operation::Square => value * value
    };
}

fn sling_stuff(m: &Monkey) -> Vec<(usize, i64)> {
    m.items.iter().map(|item| {
       let mut new_item = perform_operation(&m.op, *item);

        // println!("Monkey {}", m.num);
        // println!("\t{} -> {}", *item, new_item);

        new_item = new_item / 3;
        // print!("\t Bored to {} ", new_item);
        // print!("\t Divisible by {} ", m.test);

        let target_monkey = if new_item % m.test == 0 {
            // println!("true: Move to {}", m.when_true);
            m.when_true
        } else {
            // println!("false: Move to {}", m.when_false);
            m.when_false
        };

        (target_monkey as usize, new_item)
    }).collect()
}

fn stuff_slinging_simian_shenanigans(monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = monkeys.iter().cloned().collect();
    let mut collected_moves: Vec<(usize, i64)> = vec![];

    for m in &mut result {
        for (t, i) in &collected_moves {
            if *t == m.num {
                m.add_item(*i);
            }
        }

        m.inspections += m.items.len();
        collected_moves = collected_moves.into_iter()
            .filter(|(t, _)| m.num != *t).collect();

        let mut moves = sling_stuff(m);
        collected_moves.append(&mut moves);

        m.clear();
    }

    for m in &mut result {
        for (t, i) in &collected_moves {
            if *t == m.num {
                m.add_item(*i);
            }
        }
    }

    result
}


fn main() {
    let data = commons::read_input("input.txt");

    let mut monkeys = parse(&data);

    for i in 0..20 {
        monkeys = stuff_slinging_simian_shenanigans(monkeys);
    }

    for m in &monkeys {
        print!("Monkey {}", m.num);
        println!("\tInspections: {}", m.inspections);

        // println!("\tItems: {}", m.items.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", "));
        // println!("\tOperation: new = old {}", match m.op {
        //     Operation::Multiply(o) => format!("* {o}"),
        //     Operation::Plus(o) => format!("+ {o}"),
        //     Operation::Square => String::from("* old")
        // });
        //
        // println!("\tTest: divisible by {}", m.test);
        // println!("\t\tIf true: throw to monkey {}", m.when_true);
        // println!("\t\tIf false: throw to monkey {}", m.when_false);
    }
}
