use aoc::parse_lines;
use std::collections::VecDeque;

const OPENS: [char; 4] = ['<', '{', '[', '('];
const CLOSES: [char; 4] = ['>', '}', ']', ')'];

fn calculate_line_corruption(line: &String) -> usize {
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
    0
}

fn solve_a(lines: &Vec<String>) -> usize {
    lines.iter().map(|l| calculate_line_corruption(l)).sum()
}

pub fn day_10() {
    let input = parse_lines("day_10".to_string());
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_a(&input));
}
