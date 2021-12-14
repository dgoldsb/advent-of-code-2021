use aoc::read_file;
use std::collections::HashMap;

fn parse_rules(input: &String) -> HashMap<(char, char), char> {
    let mut rules = HashMap::new();
    for line in input.split("\n") {
        let mut split = line.chars();
        rules.insert(
            (split.nth(0).unwrap(), split.nth(0).unwrap()),
            split.nth(4).unwrap(),
        );
    }
    rules
}

fn solve(template: &String, rules: &HashMap<(char, char), char>, part_a: bool) -> usize {
    let mut current_string = template.chars().collect::<Vec<char>>();

    for i in 0..40 {
        if i == 10 && part_a {
            break;
        }

        current_string.push('-');
        current_string = current_string
            .windows(2)
            .map(|t| match rules.get(&(t[0], t[1])) {
                Some(v) => vec![t[0], *v],
                _ => vec![t[0]],
            })
            .fold(Vec::new(), |mut v, a| {
                v.extend(a);
                v
            });

    }

    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for c in current_string {
        match frequencies.get_mut(&c) {
            Some(v) => *v += 1,
            _ => { frequencies.insert(c, 1); },
        };
    }

    frequencies.values().max().unwrap() - frequencies.values().min().unwrap()
}

pub fn day_14() -> (usize, usize) {
    let file = read_file("day_14".to_string());
    let mut file_split = file.split("\n\n");
    let template = file_split.next().unwrap().to_string();
    let rules = parse_rules(&file_split.next().unwrap().to_string());
    (
        solve(&template, &rules, true),
        solve(&template, &rules, false),
    )
}
