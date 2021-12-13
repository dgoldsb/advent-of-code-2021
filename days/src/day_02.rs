use aoc::parse_lines;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Operation {
    Up(usize),
    Down(usize),
    Forward(usize),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        let mut split = input.split(" ");
        let op = split.next().unwrap();
        let val: usize = split.next().unwrap().parse().unwrap();
        match op {
            "up" => Ok(Operation::Up(val)),
            "down" => Ok(Operation::Down(val)),
            "forward" => Ok(Operation::Forward(val)),
            _ => Err(()),
        }
    }
}

fn part_a(operations: &Vec<Operation>) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;

    for operation in operations {
        match operation {
            Operation::Forward(i) => x += i,
            Operation::Down(i) => y += i,
            Operation::Up(i) => y -= i,
        }
    }
    x * y
}

fn part_b(operations: &Vec<Operation>) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut aim: usize = 0;

    for operation in operations {
        match operation {
            Operation::Forward(i) => {
                x += i;
                y += aim * i;
            }
            Operation::Down(i) => aim += i,
            Operation::Up(i) => aim -= i,
        }
    }
    x * y
}

pub fn day_02()  -> (usize, usize) {
    let instructions = parse_lines("day_02".to_string());
    let operations = instructions
        .iter()
        .map(|i| Operation::from_str(i).unwrap())
        .collect();
    (part_a(&operations), part_b(&operations))
}
