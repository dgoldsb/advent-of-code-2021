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
    route: &Vec<&String>,
    input: &HashMap<String, HashSet<String>>,
    allow_single_double: bool,
) -> usize {
    let current = route.last().unwrap();
    if **current == "end".to_string() {
        return 1;
    }

    let mut paths = 0;
    for next in input.get(*current).unwrap() {
        let is_start = *next == "start".to_string();
        let is_upper = next.to_uppercase() == *next;
        if is_upper || !route.contains(&next) || (allow_single_double && !is_start) {
            let mut allow_another_double = allow_single_double;
            if !is_upper && route.contains(&next) {
                allow_another_double = false;
            }

            let mut cloned_vec = route.clone();
            cloned_vec.push(next);
            let result = find_paths(&cloned_vec, input, allow_another_double);
            paths += result;
        }
    }

    return paths;
}

fn solve(input: &HashMap<String, HashSet<String>>, allow_single_double: bool) -> usize {
    let start = "start".to_string();
    find_paths(&vec![&start], input, allow_single_double)
}

pub fn day_12() -> (usize, usize) {
    let input = parse_input();
    (solve(&input, false), solve(&input, true))
}
