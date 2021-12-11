use aoc::parse_lines;
use std::collections::HashMap;
use std::collections::HashSet;

// TODO: Add to public module, this is duplicate code.
fn deserialize(input: &Vec<Vec<u32>>) -> HashMap<(i32, i32), u32> {
    let mut map = HashMap::new();
    for (i, l) in input.iter().enumerate() {
        for (j, v) in l.iter().enumerate() {
            map.insert((i as i32, j as i32), v.clone());
        }
    }
    map
}

fn flash(k: &(i32, i32), old: &mut HashMap<(i32, i32), u32>, flashes: &mut HashSet<(i32, i32)>) {
    old.insert(*k, 0);
    flashes.insert(k.clone());
    for d in vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ] {
        let dk = (k.0 + d.0, k.1 + d.1);
        if !flashes.contains(&dk) && old.contains_key(&dk) {
            old.insert(dk, *old.get(&dk).unwrap() + 1);
        }
    }
}

fn do_step(old: &mut HashMap<(i32, i32), u32>) -> usize {
    let keys = old.keys().map(|v| v.clone()).collect::<Vec<(i32, i32)>>();

    // First, the energy level of each octopus increases by 1.
    for k in &keys {
        old.insert(*k, *old.get(k).unwrap() + 1);
    }

    // Handle flashes until nothing can flash anymore. Octopodes can only flash once per turn, then
    // are fixed to value `0`.
    let mut flashes = HashSet::new();
    let mut old_flash_count = usize::MAX;
    while old_flash_count != flashes.len() {
        old_flash_count = flashes.len();

        for k in &keys {
            if *old.get(k).unwrap() > 9 {
                flash(k, old, &mut flashes);
            }
        }
    }

    flashes.len()
}

fn solve(input: &Vec<Vec<u32>>, part_a: bool) -> usize {
    let mut current_state = deserialize(input);

    let mut flashes = 0;
    let mut i = 0;
    while i < 100 || !part_a {
        i += 1;
        let df = do_step(&mut current_state);
        if df == 100 {
            return i;
        }
        flashes += df;
    }
    return flashes;
}

pub fn day_11() {
    let input = parse_lines("day_11".to_string())
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    println!("A: {}", solve(&input, true));
    println!("B: {}", solve(&input, false));
}
