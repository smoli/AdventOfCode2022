use regex::Regex;
use crate::Operation::{Multiply, Plus};

enum Operation {
    Multiply(i32),
    Plus(i32),
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

struct Monkey {
    num: usize,
    items: Vec<i32>,
    op: Operation,
    test: i32,
    when_true: i32,
    when_false: i32,
    inspections: u32,
}

impl Monkey {
    pub fn add_item(&mut self, item: i32) {
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

                _ => Operation::Multiply(opc.get(2).unwrap().as_str().parse::<i32>().unwrap() as i32)
            },

            "+" => Operation::Plus(opc.get(2).unwrap().as_str().parse::<i32>().unwrap() as i32),

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


fn perform_operation(op: &Operation, value: i32) -> i32 {
    return match op {
        Multiply(v) => value * v,
        Operation::Plus(v) => value + v,
        Operation::Square => value * value
    };
}

fn sling_stuff(m: &Monkey) -> Vec<(usize, i32)> {
    m.items.iter().map(|item| {
        let mut new_item = perform_operation(&m.op, *item);
        new_item = (new_item as f32 / 3.0).floor() as i32;
        let target_monkey = if new_item % m.test == 0 {
            m.when_true
        } else {
            m.when_false
        };

        (target_monkey as usize, new_item)
    }).collect()
}

fn stuff_slinging_simian_shenanigans(monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let mut result: Vec<Monkey> = monkeys.iter().cloned().collect();
    let mut collected_moves: Vec<(usize, i32)> = vec![];

    for m in &mut result {
        for (t, i) in &collected_moves {
            if *t == m.num {
                m.add_item(*i);
            }
        }
        collected_moves = collected_moves.into_iter()
            .filter(|(t, _)| m.num != *t).collect();

        let mut moves = sling_stuff(m);
        collected_moves.append(&mut moves);

        m.inspections += m.items.len() as u32;
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
    let data = commons::read_input("sampleInput.txt");

    let mut monkeys = parse(&data);

    for _ in 0..20 {
        monkeys = stuff_slinging_simian_shenanigans(monkeys);
    }

    for m in &monkeys {
        print!("Monkey {}", m.num);
        println!("\tInspections: {}", m.inspections);

        println!("\tItems: {}", m.items.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", "));
        println!("\tOperation: new = old {}", match m.op {
            Operation::Multiply(o) => format!("* {o}"),
            Operation::Plus(o) => format!("+ {o}"),
            Operation::Square => String::from("* old")
        });

        println!("\tTest: divisible by {}", m.test);
        println!("\t\tIf true: throw to monkey {}", m.when_true);
        println!("\t\tIf false: throw to monkey {}", m.when_false);
    }
}
