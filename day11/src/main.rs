use regex::Regex;
use crate::Operation::Multiply;

enum Operation {
    Multiply(i32),
    Plus(i32),
    Square
}

struct Monkey {
    start: Vec<i32>,
    op: Operation,
    test: i32,
    when_true: i32,
    when_false: i32
}


fn parse(input: &Vec<String>) -> Vec<Monkey> {
    let mut result:Vec<Monkey> = vec![];
    let r_monkey = Regex::new(r"^Monkey (\d+):").unwrap();
    let r_start = Regex::new(r"^\s+Starting items: (.*)").unwrap();
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

        let monkey_num = r_monkey.captures(l).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap() as u32;

        i += 1;
        l = input.get(i).unwrap();
        let start = r_start.captures(l).unwrap()
                            .get(1).unwrap().as_str()
                            .split(", ").collect::<Vec<&str>>()
                            .into_iter().map(|s| s.parse().unwrap()).collect();

        i+= 1;
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
            start,
            op,
            test,
            when_true,
            when_false,
        });

        i += 2;
    }


    result
}


fn main() {
    let data = commons::read_input("sampleInput.txt");

    let monkeys = parse(&data);

    for m in monkeys {
        println!("Monkey");
        println!("\tStarting items: {}", m.start.into_iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", "));
        println!("\tOperation: new = old {}", match m.op {
            Operation::Multiply(o) => format!("* {o}"),
            Operation::Plus(o) => format!("+ {o}"),
            Operation::Square => String::from("old * old")
        });

        println!("\tTest: divisible by {}", m.test);
        println!("\t\tIf true: throw to monkey {}", m.when_true);
        println!("\t\tIf false: throw to monkey {}", m.when_false);
    }

    println!("Hello, world!");
}
