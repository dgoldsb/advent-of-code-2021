use aoc::parse_lines;
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input() -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    let lines = parse_lines("day_12".to_string());
    for line in lines {
        let split_line = line
            .split("-")
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        let first = split_line.get(0).unwrap();
        let second = split_line.get(1).unwrap();

        map.entry(first.clone()).or_default().insert(second.clone());
        map.entry(second.clone()).or_default().insert(first.clone());
    }
    return map;
}

fn find_paths(
    route: &Vec<String>,
    input: &HashMap<String, HashSet<String>>,
    allow_single_double: bool,
) -> Vec<Vec<String>> {
    let current = route.last().unwrap();
    if current.clone() == "end".to_string() {
        return vec![route.clone()];
    }

    let mut paths = Vec::new();
    for next in input.get(current).unwrap() {
        let is_start = next.clone() == "start".to_string();
        let is_upper = next.to_uppercase() == next.clone();
        if is_upper || !route.contains(next) || (allow_single_double && !is_start) {
            let mut allow_another_double = allow_single_double;
            if !is_upper && route.contains(next) {
                allow_another_double = false;
            }

            let mut cloned_vec = route.clone();
            cloned_vec.push(next.clone());
            let result = find_paths(&cloned_vec, input, allow_another_double);
            paths.extend(result.iter().map(|v| v.clone()));
        }
    }

    return paths;
}

fn solve(input: &HashMap<String, HashSet<String>>, allow_single_double: bool) -> usize {
    find_paths(&vec!["start".to_string()], input, allow_single_double).len()
}

pub fn day_12() {
    let input = parse_input();
    println!("A: {}", solve(&input, false));
    println!("B: {}", solve(&input, true));
}
