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
    current_string.push('x'); // mark the tail

    let mut window_count: HashMap<(char, char), usize> = HashMap::new();
    current_string.windows(2).for_each(|w| {
        let t = (w[0], w[1]);
        match window_count.get_mut(&t) {
            Some(v) => *v += 1,
            _ => {
                window_count.insert(t, 1);
            }
        };
    });

    for i in 0..40 {
        if i == 10 && part_a {
            break;
        }

        let mut new_window_count: HashMap<(char, char), usize> = HashMap::new();

        for (t, c) in window_count {
            match rules.get(&t) {
                Some(v) => {
                    let t1 = (t.0, *v);
                    let t2 = (*v, t.1);
                    match new_window_count.get_mut(&t1) {
                        Some(v) => *v += c,
                        _ => {
                            new_window_count.insert(t1, c);
                        }
                    };
                    match new_window_count.get_mut(&t2) {
                        Some(v) => *v += c,
                        _ => {
                            new_window_count.insert(t2, c);
                        }
                    };
                }
                _ => {
                    match new_window_count.get_mut(&t) {
                        Some(v) => *v += c,
                        _ => {
                            new_window_count.insert(t, c);
                        }
                    };
                }
            };
        }

        window_count = new_window_count;
    }

    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for (t, c) in window_count {
        match frequencies.get_mut(&t.0) {
            Some(v) => *v += c,
            _ => {
                frequencies.insert(t.0, c);
            }
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
