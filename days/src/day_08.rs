use aoc::parse_lines;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Entry {
    signal_pattern: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(input: &str) -> Result<Entry, Self::Err> {
        let mut split = input.split(" | ");
        let signal_pattern = split
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        let output = split
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        return Ok(Entry {
            signal_pattern,
            output,
        });
    }
}

fn solve_a(entries: &Vec<Entry>) -> usize {
    let filter: HashSet<usize> = [2, 3, 4, 7].iter().cloned().collect();
    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .filter(|s| filter.contains(&s.len()))
                .collect::<Vec<&HashSet<char>>>()
                .len()
        })
        .sum()
}

pub fn day_08() {
    let entries = parse_lines("day_08".to_string())
        .iter()
        .map(|s| Entry::from_str(s).unwrap())
        .collect::<Vec<Entry>>();
    println!("A: {}", solve_a(&entries));
    println!("B: {}", solve_a(&entries));
}
