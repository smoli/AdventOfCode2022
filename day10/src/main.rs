extern crate core;

use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use regex::Regex;

enum Operation {
    Noop,
    Add(String, i32),
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Operation::Noop => write!(f, "Noop"),
            Operation::Add(a, b) => write!(f, "Add {} to {}", b, a)
        }
    }
}

fn get_cycles_per_op(op: &Operation) -> u8 {
    return match op {
        Operation::Noop => 1,
        Operation::Add(_, _) => 2
    };
}

type Registers = HashMap<String, i32>;

struct Protocolentry {
    cycle: u16,
    registers: Registers,
}

impl Display for Protocolentry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, x: {}", self.cycle, self.registers.get("x").unwrap())
    }
}

fn perform_operation(op: &Operation, mut registers: Registers) -> Registers {
    return match op {
        Operation::Noop => registers,

        Operation::Add(r, v) => {
            registers.insert(r.clone(), registers.get(r).unwrap() + v);
            registers
        }
    };
}


fn cpu_loop(input: &Vec<Operation>, failSafe: i32) -> Vec<Protocolentry> {
    let mut log: Vec<Protocolentry> = vec![];
    let mut count = failSafe;
    let mut pc:i32 = -1;
    let mut cycles_remaining = 0;
    let mut registers: Registers = HashMap::new();
    let mut cycle = 0;

    registers.insert(String::from("x"), 1);

    while count > 0 {
        cycle += 1;
        if cycles_remaining == 0 {
            if pc >= 0 {
                registers = perform_operation(input.get(pc as usize).unwrap(), registers)
            }

            let mut logreg: Registers = HashMap::new();
            logreg.clone_from(&registers);
            log.push(Protocolentry { cycle, registers: logreg });

            pc += 1;
            if (pc as usize) >= input.len() {
                break;
            }

            let op = input.get(pc as usize).unwrap();
            cycles_remaining = get_cycles_per_op(op);

        } else {
            let mut logreg: Registers = HashMap::new();
            logreg.clone_from(&registers);
            log.push(Protocolentry { cycle, registers: logreg });
        }
        cycles_remaining -= 1;
        count -= 1;
    }

    log
}

fn parse(input: &Vec<String>) -> Vec<Operation> {
    let mut result: Vec<Operation> = vec![];

    let op = Regex::new(r"^(add|noop)(\w)?\s?(-?\d*)").unwrap();

    for line in input {
        let c = op.captures(line).unwrap();

        let command = c.get(1).unwrap().as_str();

        match command {
            "noop" => result.push(Operation::Noop),

            "add" => result.push(Operation::Add(
                String::from(c.get(2).unwrap().as_str()),
                c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            )),

            _ => panic!()
        }
    }

    result
}


fn main() {
    let data = commons::read_input("input.txt");

    let programm = parse(&data);

    let log = cpu_loop(&programm, 1000);

    let mut sum = 0;

    for l in &log {
        if l.cycle >= 20 && l.cycle <= 220 && (l.cycle - 20) % 40 == 0 {
            sum += l.registers.get("x").unwrap() * l.cycle as i32;
        }
    }

    println!("Part One: {}", sum);

}
