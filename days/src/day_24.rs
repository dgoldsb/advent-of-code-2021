use aoc::parse_lines;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Inp(char),
    Add(char, char),
    Mul(char, char),
    Div(char, char),
    Mod(char, char),
    Eql(char, char),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let mut split = input.split(" ");
        let op = split.next().unwrap();
        let val: char = split.next().unwrap().parse().unwrap();
        match op {
            "inp" => Ok(Instruction::Inp(val)),
            "add" => Ok({
                let val_val: char = split.next().unwrap().parse().unwrap();
                Instruction::Add(val, val_val)
            }),
            "mul" => Ok({
                let val_val: char = split.next().unwrap().parse().unwrap();
                Instruction::Mul(val, val_val)
            }),
            "div" => Ok({
                let val_val: char = split.next().unwrap().parse().unwrap();
                Instruction::Div(val, val_val)
            }),
            "mod" => Ok({
                let val_val: char = split.next().unwrap().parse().unwrap();
                Instruction::Mod(val, val_val)
            }),
            "eql" => Ok({
                let val_val: char = split.next().unwrap().parse().unwrap();
                Instruction::Eql(val, val_val)
            }),
            _ => Err(()),
        }
    }
}

fn get_or(v: &char, memory: &HashMap<char, isize>) -> isize {
    match memory.get(v) {
        Some(x) => *x,
        _ => (*v) as isize - 48, // I know
    }
}

// Returns w, x, y, z
fn run_program(mut inputs: Vec<isize>, program: &Vec<Instruction>) -> (isize, isize, isize, isize) {
    let mut memory: HashMap<char, isize> = vec![('w', 0), ('x', 0), ('y', 0), ('z', 0)]
        .into_iter()
        .collect::<HashMap<char, isize>>();

    // Iterate over the program.
    for instruction in program {
        match instruction {
            Instruction::Inp(a) => memory.insert(*a, inputs.remove(0)),
            Instruction::Add(a, b) => memory.insert(*a, get_or(&a, &memory) + get_or(&b, &memory)),
            Instruction::Mul(a, b) => memory.insert(*a, get_or(&a, &memory) * get_or(&b, &memory)),
            Instruction::Div(a, b) => memory.insert(*a, get_or(&a, &memory) / get_or(&b, &memory)),
            Instruction::Mod(a, b) => memory.insert(*a, get_or(&a, &memory) % get_or(&b, &memory)),
            Instruction::Eql(a, b) => memory.insert(*a, {
                if get_or(&a, &memory) == get_or(&b, &memory) {
                    1
                } else {
                    0
                }
            }),
        };
    }

    (
        *memory.get(&'w').unwrap(),
        *memory.get(&'x').unwrap(),
        *memory.get(&'y').unwrap(),
        *memory.get(&'z').unwrap(),
    )
}

fn solve(program: &Vec<Instruction>, _part_a: bool) -> usize {
    // Stoopid but let's go.
    let mut start: isize = 99999999999999;

    loop {
        // To string.
        let current: String = start.to_string();

        // Continue if has a zero.
        if current.contains('0') {
            continue;
        }

        // Pad.
        let padded_current = format!("{:0>14}", current);

        // Check output.
        let input = padded_current
            .chars()
            .map(|c| (c as isize) - 48)
            .collect::<Vec<isize>>();
        let result = run_program(input, program);
        if result.3 == 0 {
            return start as usize;
        }

        // Minus 1.
        start -= 1;
    }
}

pub fn day_24() -> (usize, usize) {
    let input = parse_lines("day_24".to_string());
    let program = input
        .iter()
        .map(|s| Instruction::from_str(&s).unwrap())
        .collect::<Vec<Instruction>>();
    (solve(&program, true), solve(&program, false))
}

#[cfg(test)]
mod tests {
    use crate::day_24::run_program;
    use crate::day_24::Instruction;

    #[test]
    fn example_1() {
        let result = run_program(
            vec![1, 3],
            &vec![
                Instruction::Inp('z'),
                Instruction::Inp('x'),
                Instruction::Mul('z', '3'),
                Instruction::Eql('x', 'z'),
            ],
        );
        assert_eq!(result.3, 1);
    }

    #[test]
    fn example_2() {
        let result = run_program(
            vec![1, 2],
            &vec![
                Instruction::Inp('z'),
                Instruction::Inp('x'),
                Instruction::Mul('z', '3'),
                Instruction::Eql('x', 'z'),
            ],
        );
        assert_eq!(result.3, 0);
    }
}
