use aoc::from_bin;
use aoc::parse_lines;
use std::collections::HashSet;

fn tally_to_binary_char(number: &isize, invert: bool) -> char {
    if (number == &0) && invert {
        '1'
    } else if number == &0 {
        '0'
    } else if (number > &0) ^ !invert {
        '1'
    } else if (number < &0) ^ !invert {
        '0'
    } else {
        panic!("unknown case")
    }
}

fn part_a(instructions: &Vec<String>) -> usize {
    let mut array: [isize; 12] = [0; 12];

    for instruction in instructions {
        for (i, c) in instruction.chars().enumerate() {
            match c {
                '1' => array[i] += 1,
                '0' => array[i] -= 1,
                _ => {
                    panic!("unknown case")
                }
            }
        }
    }

    let gamma = from_bin(
        &array
            .iter()
            .map(|s| tally_to_binary_char(s, false))
            .collect::<Vec<char>>(),
    );
    let epsilon = from_bin(
        &array
            .iter()
            .map(|s| tally_to_binary_char(s, true))
            .collect::<Vec<char>>(),
    );
    gamma * epsilon
}

fn recursive_filter(instruction_set: &HashSet<String>, index: usize, invert: bool) -> String {
    let mut tally: isize = 0;

    for instruction in instruction_set {
        match instruction.chars().nth(index).unwrap() {
            '1' => tally += 1,
            '0' => tally -= 1,
            _ => {
                panic!("unknown case")
            }
        }
    }

    let target_char = tally_to_binary_char(&tally, invert);
    let mut new_set: HashSet<String> = HashSet::new();

    for instruction in instruction_set {
        if instruction.chars().nth(index).unwrap() == target_char {
            new_set.insert(instruction.to_string());
        }
    }

    if new_set.len() == 1 {
        new_set.into_iter().next().unwrap()
    } else {
        recursive_filter(&new_set, index + 1, invert)
    }
}

fn part_b(instructions: &Vec<String>) -> usize {
    let instruction_set: HashSet<String> = instructions
        .iter()
        .map(|s| s.clone())
        .collect::<HashSet<String>>();
    let oxygen = from_bin(
        &recursive_filter(&instruction_set, 0, true)
            .chars()
            .collect::<Vec<char>>(),
    );
    let co2 = from_bin(
        &recursive_filter(&instruction_set, 0, false)
            .chars()
            .collect::<Vec<char>>(),
    );
    oxygen * co2
}

pub fn day_03() -> (usize, usize) {
    let instructions = parse_lines("day_03".to_string());
    (part_a(&instructions), part_b(&instructions))
}
