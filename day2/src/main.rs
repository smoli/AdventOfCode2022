use std::{fmt, fs};
use std::fmt::write;
use std::str::Split;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RPS::Rock => write!(f, "Rock"),
            RPS::Scissors => write!(f, "Scissors"),
            RPS::Paper => write!(f, "Paper")
        }
    }
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn round_score(result: Result, own_choice: RPS) -> i32 {
    let choice_score = match own_choice {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    };

    let result_score = match result {
        Result::Draw => 3,
        Result::Lose => 0,
        Result::Win => 6
    };

    return choice_score + result_score;
}

fn opponent_choice(choice: &str) -> Option<RPS> {
    match choice {
        "A" => Some(RPS::Rock),
        "B" => Some(RPS::Paper),
        "C" => Some(RPS::Scissors),
        _ => None
    }
}

fn own_choice(choice: &str) -> Option<RPS> {
    match choice {
        "X" => Some(RPS::Rock),
        "Y" => Some(RPS::Paper),
        "Z" => Some(RPS::Scissors),
        _ => None
    }
}

fn map_result(expectation: &str) -> Option<Result> {
    match expectation {
        "X" => Some(Result::Lose),
        "Y" => Some(Result::Draw),
        "Z" => Some(Result::Win),
        _ => None
    }
}

fn desired_choice(opponent_choice: RPS, expected_result: Result) -> RPS {
    return match expected_result {

        Result::Lose => match opponent_choice {
                RPS::Rock => RPS::Scissors,

                RPS::Paper => RPS::Rock,

                RPS::Scissors => RPS::Paper
            }
        Result::Draw => opponent_choice,

        Result::Win => match opponent_choice {
            RPS::Rock => RPS::Paper,

            RPS::Paper => RPS::Scissors,

            RPS::Scissors => RPS::Rock
        }
    }
}

fn fight(opponent_choice: RPS, own_choice: RPS) -> i32 {
    let mut winning = Result::Draw;

    match opponent_choice {
        RPS::Rock =>
            match own_choice {
                RPS::Rock => winning = Result::Draw,
                RPS::Paper => winning = Result::Win,
                RPS::Scissors => winning = Result::Lose,
            },

        RPS::Paper =>
            match own_choice {
                RPS::Rock => winning = Result::Lose,
                RPS::Paper => winning = Result::Draw,
                RPS::Scissors => winning = Result::Win,
            },

        RPS::Scissors =>
            match own_choice {
                RPS::Rock => winning = Result::Win,
                RPS::Paper => winning = Result::Lose,
                RPS::Scissors => winning = Result::Draw,
            }
    }

    round_score(winning, own_choice)
}

fn read_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("File not found");

    let lines = contents.split("\r\n");

    let mut r: Vec<String> = vec![];
    for line in lines {
        r.push(String::from(line));
    }

    return r;
}


fn main() {
    let data = read_input("sampleInput.txt");

    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;

    for battle in data {
        let choices = battle.split(" ").collect::<Vec<&str>>();
        sum_1 += fight(opponent_choice(choices[0]).unwrap(),
                       own_choice(choices[1]).unwrap());

        sum_2 += fight(opponent_choice(choices[0]).unwrap(),
                       desired_choice(opponent_choice(choices[0]).unwrap(),
                                      map_result(choices[1]).unwrap()));
    }

    println!("Your Score: {}", sum_1);
    println!("Your Score: {}", sum_2);
}
