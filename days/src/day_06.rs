use aoc::parse_ints;
use std::collections::HashMap;

fn solve(inputs: &Vec<isize>, duration: isize) -> usize {
    let mut fish = HashMap::new();

    for input in inputs {
        let count = fish.entry(*input).or_insert(0);
        *count += 1;
    }

    for _ in 0..duration {
        let mut new_fish = HashMap::new();
        for key in 0..=8 {
            let fish_count = fish.entry(key).or_insert(0).clone();

            // Move cohorts around.
            if key == 0 {
                // Parents are reset to 6.
                let new_fish_count = new_fish.entry(6).or_insert(0);
                *new_fish_count += fish_count;

                // Children are set to 8
                let new_fish_count = new_fish.entry(8).or_insert(0);
                *new_fish_count += fish_count;
            } else {
                let new_fish_count = new_fish.entry(key - 1).or_insert(0);
                *new_fish_count += fish_count;
            }
        }
        fish = new_fish;
    }

    fish.values().sum()
}

pub fn day_06() {
    let inputs = parse_ints("day_06".to_string());
    println!("A: {}", solve(&inputs, 80));
    println!("B: {}", solve(&inputs, 256));
}
