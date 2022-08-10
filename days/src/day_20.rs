use aoc::from_bin;
use aoc::parse_items;
use std::collections::HashSet;

fn get_neighborhood(input: &(isize, isize)) -> Vec<(isize, isize)> {
    let mut neighborhood = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            // Ordering is important!
            neighborhood.push((input.0 + i, input.1 + j));
        }
    }
    neighborhood
}

fn will_be_lit(
    input: &(isize, isize),
    lit: &HashSet<(isize, isize)>,
    unlit: &HashSet<(isize, isize)>,
    algorithm: &Vec<char>,
    default_on: &bool,
) -> bool {
    let mut bin = Vec::new();
    for neighbor in get_neighborhood(input) {
        if *default_on {
            if unlit.contains(&neighbor) {
                bin.push('0');
            } else {
                bin.push('1');
            }
        } else {
            if lit.contains(&neighbor) {
                bin.push('1');
            } else {
                bin.push('0');
            }
        }
    }

    let index = from_bin(&bin);
    return algorithm.get(index).unwrap() == &'#';
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    // Parse the input into mutable hash sets.
    let algorithm = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let lines = input
        .get(1)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut lit: HashSet<(isize, isize)> = HashSet::new();
    let mut unlit: HashSet<(isize, isize)> = HashSet::new();
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        for j in 0..(line.len() + 2) {
            let c = line.chars().nth(j).unwrap_or('.');
            match c {
                '#' => {
                    lit.insert((i as isize, j as isize));
                }
                '.' => {
                    unlit.insert((i as isize, j as isize));
                }
                _ => {
                    panic!("non-binary pixel {}", c)
                }
            };
        }
    }

    let mut part_a = 0;
    for i in 0..50 {
        if i == 2 {
            part_a = lit.len();
        }

        let default_on = (i % 2 == 1) && algorithm.get(0).unwrap() == &'#';
        // Get all candidates to be lit.
        let candidates = lit
            .union(&unlit)
            .map(|c| get_neighborhood(c))
            .flatten()
            .collect::<HashSet<(isize, isize)>>();

        // Evaluate candidates against neighbors and the algorithm.
        let filtered_candidates = candidates
            .iter()
            .filter(|c| will_be_lit(c, &lit, &unlit, &algorithm, &default_on))
            .map(|c| *c)
            .collect::<HashSet<(isize, isize)>>();

        // Replace.
        lit = filtered_candidates;
        unlit = candidates
            .difference(&lit)
            .map(|c| *c)
            .collect::<HashSet<(isize, isize)>>();
    }
    return (part_a, lit.len());
}

pub fn day_20() -> (usize, usize) {
    let input = parse_items("day_20".to_string(), "\n\n".to_string());
    solve(&input)
}
