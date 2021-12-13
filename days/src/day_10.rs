use aoc::parse_lines;
use std::collections::VecDeque;

const OPENS: [char; 4] = ['<', '{', '[', '('];
const CLOSES: [char; 4] = ['>', '}', ']', ')'];

fn calculate_line_value(line: &String) -> isize {
    let mut stack: VecDeque<char> = VecDeque::new();
    for ch in line.chars() {
        if OPENS.contains(&ch) {
            stack.push_back(match ch {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => '_',
            });
        } else if CLOSES.contains(&ch) {
            if stack.pop_back().unwrap() != ch {
                return match ch {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
            }
        }
    }
    let mut value = 0;
    while !stack.is_empty() {
        value *= 5;
        value += match stack.pop_back().unwrap() {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }
    value * -1
}

fn solve_a(lines: &Vec<String>) -> isize {
    lines
        .iter()
        .map(|l| calculate_line_value(l))
        .filter(|v| *v > 0)
        .sum()
}

fn solve_b(lines: &Vec<String>) -> isize {
    let mut values = lines
        .iter()
        .map(|l| calculate_line_value(l))
        .filter(|v| *v < 0)
        .collect::<Vec<isize>>();
    values.sort();
    values.get(values.len() / 2).unwrap() * -1
}

pub fn day_10() -> (usize, usize) {
    let input = parse_lines("day_10".to_string());
    (solve_a(&input) as usize, solve_b(&input) as usize)
}
