use aoc::parse_items;
use std::collections::HashSet;

// Flishy flashy: index 0 at even round numbers, index 111111111 when odd.
// Assumption: we only

fn solve(input: &Vec<String>) -> (usize, usize) {
    let algorithm = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let lines = input.get(1).unwrap().split("/n").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut lit: HashSet<(isize, isize)> = HashSet::new();
    let mut unlit: HashSet<(isize, isize)> = HashSet::new();

    let default_vec: String = String::new();
    for i in 0..(lines.len() + 2) {
        let si: isize = i as isize - 1;
        let mut line;
        if si < 0 {
            line = String::new();
        } else {
            line = lines.get(si as usize).unwrap_or(&default_vec).clone();
        }

        for j in 0..(line.len() + 2) {
            let sj = j as isize - 1;
            let c;
            if sj < 0 {
                c = '.';
            } else {
                c = line.chars().nth(sj as usize).unwrap_or('.');
            }
            match c {
                '#' => {lit.insert((si, sj));},
                '.' => {unlit.insert((si, sj));},
                _ => {panic!("non-binary pixel")},
            };
        }
    }

    println!("{}", algorithm.len());
    println!("{}", lit.len());
    println!("{}", unlit.len());

    let mut default_on = false;
    let part_a = 0;
    let part_b = 0;
    for i in 0..50 {
        panic!("");
    }
    return (part_a, part_b);
}

pub fn day_20() -> (usize, usize) {
    let input = parse_items("day_20".to_string(), "/n/n".to_string());
    solve(&input)
}
