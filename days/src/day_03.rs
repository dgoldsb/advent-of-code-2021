use aoc::from_bin;
use aoc::parse_lines;

fn tally_to_binary_char(number: &isize, invert: bool) -> char {
    if (number > &0) ^ !invert {
        '1'
    } else if (number < &0) ^ !invert {
        '0'
    } else {
        panic!("drew even!")
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

    println!("{:?}", array);

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

pub fn day_03() {
    let instructions = parse_lines("day_03".to_string());
    println!("A: {}", part_a(&instructions));
    println!("B: {}", part_a(&instructions));
}
