use aoc::parse_u32_map;
use std::collections::HashMap;
use std::collections::HashSet;

fn is_low_point(i: i32, j: i32, map: &HashMap<(i32, i32), u32>) -> bool {
    for deltas in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
        if map.get(&(i + deltas.0, j + deltas.1)).unwrap_or(&&u32::MAX) <= map.get(&(i, j)).unwrap()
        {
            return false;
        }
    }
    true
}

fn solve_a(map: &HashMap<(i32, i32), u32>) -> u32 {
    map.keys()
        .filter(|k| is_low_point(k.0, k.1, map))
        .map(|k| *map.get(k).unwrap() + 1)
        .sum()
}

fn flood_fill(
    k: &(i32, i32),
    basin_map: &mut HashMap<(i32, i32), i32>,
    map: &HashMap<(i32, i32), u32>,
    basin_identifier: i32,
) {
    if basin_map.contains_key(k) {
        return;
    } else if *map.get(k).unwrap_or(&&9) == 9 {
        basin_map.insert(*k, i32::MAX);
    } else {
        basin_map.insert(*k, basin_identifier);
        for deltas in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if map.contains_key(k) {
                flood_fill(
                    &(k.0 + deltas.0, k.1 + deltas.1),
                    basin_map,
                    map,
                    basin_identifier,
                );
            }
        }
    }
}

fn solve_b(map: &HashMap<(i32, i32), u32>) -> usize {
    let mut basin_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut basin_identifiers = 0..10000;

    for k in map.keys() {
        flood_fill(k, &mut basin_map, map, basin_identifiers.next().unwrap())
    }

    let mut basin_sizes: Vec<usize> = basin_map
        .values()
        .collect::<HashSet<&i32>>()
        .iter()
        .map(|b| {
            basin_map
                .values()
                .filter(|v| v == b && **v != i32::MAX)
                .count()
        })
        .collect::<Vec<usize>>();
    basin_sizes.sort();
    basin_sizes[(basin_sizes.len() - 3)..]
        .iter()
        .fold(1, |a, b| a * b)
}

pub fn day_09() -> (usize, usize) {
    let map = parse_u32_map("day_09".to_string());
    (solve_a(&map) as usize, solve_b(&map) as usize)
}
