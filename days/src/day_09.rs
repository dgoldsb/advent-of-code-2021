use aoc::parse_lines;
use std::collections::HashMap;

fn deserialize(input: &Vec<Vec<u32>>) -> HashMap<(i32, i32), &u32> {
    let mut map = HashMap::new();
    for (i, l) in input.iter().enumerate() {
        for (j, v) in l.iter().enumerate() {
            map.insert((i as i32, j as i32), v);
        }
    }
    map
}

fn is_low_point(i: i32, j: i32, map: &HashMap<(i32, i32), &u32>) -> bool {
    for deltas in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
        if map.get(&(i + deltas.0, j + deltas.1)).unwrap_or(&&u32::MAX) <= map.get(&(i, j)).unwrap()
        {
            return false;
        }
    }
    true
}

fn solve_a(map: &HashMap<(i32, i32), &u32>) -> u32 {
    map.keys()
        .filter(|k| is_low_point(k.0, k.1, map))
        .map(|k| *map.get(k).unwrap() + 1)
        .sum()
}

pub fn day_09() {
    let input = parse_lines("day_09".to_string())
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let map = deserialize(&input);
    println!("A: {}", solve_a(&map));
    println!("B: {}", solve_a(&map));
}
