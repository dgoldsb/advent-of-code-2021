use aoc::parse_lines;
use std::collections::HashSet;
use std::str::FromStr;

fn find_codeset(samples: &Vec<HashSet<char>>) -> Vec<Vec<String>> {
    let c1 = samples.iter().filter(|s| s.len() == 2).next().unwrap();
    let c7 = samples.iter().filter(|s| s.len() == 3).next().unwrap();
    let c4 = samples.iter().filter(|s| s.len() == 4).next().unwrap();
    let c8 = samples.iter().filter(|s| s.len() == 7).next().unwrap();
    // Length of 6 and `c4` is not a subset and has `c1` as a subset.
    let c0 = samples
        .iter()
        .filter(|s| s.len() == 6 && !c4.is_subset(s) && c1.is_subset(s))
        .next()
        .unwrap();
    // Length of 6 and has `c4` as a subset.
    let c9 = samples
        .iter()
        .filter(|s| s.len() == 6 && c4.is_subset(s))
        .next()
        .unwrap();
    // Length of 6 and does not have `c7` as a subset and does not have `c1` as a subset.
    let c6 = samples
        .iter()
        .filter(|s| s.len() == 6 && !c7.is_subset(s) && !c1.is_subset(s))
        .next()
        .unwrap();
    // Length of 5 and is a subset of `c9`.
    let c2 = samples
        .iter()
        .filter(|s| s.len() == 5 && !s.is_subset(c9))
        .next()
        .unwrap();
    // Length of 5 and is no a subset of `c9` and `c1` is not a subset.
    let c5 = samples
        .iter()
        .filter(|s| s.len() == 5 && s.is_subset(c9) && !c1.is_subset(s))
        .next()
        .unwrap();
    // Length of 5 and has `c1` as a subset.
    let c3 = samples
        .iter()
        .filter(|s| s.len() == 5 && c1.is_subset(s))
        .next()
        .unwrap();

    vec![
        vec![c0.iter().collect::<String>(), "0".to_string()],
        vec![c1.iter().collect::<String>(), "1".to_string()],
        vec![c2.iter().collect::<String>(), "2".to_string()],
        vec![c3.iter().collect::<String>(), "3".to_string()],
        vec![c4.iter().collect::<String>(), "4".to_string()],
        vec![c5.iter().collect::<String>(), "5".to_string()],
        vec![c6.iter().collect::<String>(), "6".to_string()],
        vec![c7.iter().collect::<String>(), "7".to_string()],
        vec![c8.iter().collect::<String>(), "8".to_string()],
        vec![c9.iter().collect::<String>(), "9".to_string()],
    ]
}

fn decode(input: &HashSet<char>, samples: &Vec<HashSet<char>>) -> char {
    let codeset: Vec<Vec<String>> = find_codeset(samples);
    // Less efficient than a hashmap lookup, but only 800 entries are to be decoded.
    for i in 0..10 {
        let code_set: HashSet<char> = codeset.get(i).unwrap().get(0).unwrap().chars().collect();
        if &code_set == input {
            return codeset
                .get(i)
                .unwrap()
                .get(1)
                .unwrap()
                .clone()
                .chars()
                .next()
                .unwrap();
        }
    }
    panic!("unknown code")
}

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

fn solve_b(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .map(|s| decode(s, &[&e.signal_pattern[..], &e.output[..]].concat()))
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

pub fn day_08() {
    let entries = parse_lines("day_08".to_string())
        .iter()
        .map(|s| Entry::from_str(s).unwrap())
        .collect::<Vec<Entry>>();
    println!("A: {}", solve_a(&entries));
    println!("B: {}", solve_b(&entries));
}
