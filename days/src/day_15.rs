use self::priority_queue::PriorityQueue;
use aoc::parse_u32_map;
use std::collections::HashMap;
use std::collections::VecDeque;

extern crate priority_queue;

fn neighbors(point: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    for deltas in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
        neighbors.push((point.0 - deltas.0, point.1 - deltas.1));
    }
    neighbors
}

fn reconstruct_path(
    path: &mut VecDeque<(i32, i32)>,
    came_from: &HashMap<(i32, i32), (i32, i32)>,
    current: &(i32, i32),
) {
    path.push_front(current.clone());

    let mut next_current = current;
    while came_from.contains_key(next_current) {
        next_current = &came_from.get(next_current).unwrap();
        path.push_front(next_current.clone());
    }
}

fn heuristic(a: &(i32, i32), b: &(i32, i32)) -> u32 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
}

fn a_star(costs: &HashMap<(i32, i32), u32>, goal: &(i32, i32)) -> VecDeque<(i32, i32)> {
    let mut open_set: PriorityQueue<(i32, i32), u32> = PriorityQueue::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), u32> = HashMap::new();
    let mut f_score: HashMap<(i32, i32), u32> = HashMap::new();

    let start = (0, 0);
    open_set.push(start, 0);
    g_score.insert(start, *costs.get(&start).unwrap());
    // F-score is inverted.
    f_score.insert(
        start,
        u32::MAX - (costs.get(&start).unwrap() + heuristic(&start, goal)),
    );

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().0;
        if &current == goal {
            let mut total_path = VecDeque::new();
            reconstruct_path(&mut total_path, &came_from, &current);
            return total_path;
        }

        for neighbor in neighbors(&current) {
            match costs.get(&neighbor) {
                Some(cost) => {
                    let tentative_g_score = g_score.get(&current).unwrap() + *cost;

                    if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                        came_from.insert(neighbor, current);
                        g_score.insert(neighbor, tentative_g_score);
                        let tentative_f_score =
                            u32::MAX - (tentative_g_score + heuristic(&neighbor, goal));
                        f_score.insert(neighbor, tentative_f_score);

                        match open_set.get_priority(&neighbor) {
                            Some(p) => {
                                if tentative_f_score > *p {
                                    open_set.push(neighbor, tentative_f_score);
                                }
                            }
                            None => {
                                open_set.push(neighbor, tentative_f_score);
                            }
                        };
                    }
                }
                _ => {}
            }
        }
    }

    panic!("Open set is empty but goal was never reached");
}

fn solve(input: &HashMap<(i32, i32), u32>) -> usize {
    a_star(&input, input.keys().max().unwrap())
        .iter()
        .map(|v| *input.get(v).unwrap() as usize)
        .sum::<usize>()
        - *input.get(&(0, 0)).unwrap() as usize
}

pub fn day_15() -> (usize, usize) {
    let input = parse_u32_map("day_15".to_string());

    let mut large_input = HashMap::new();
    let max = input.keys().max().unwrap();
    for i in 0..5 {
        for j in 0..5 {
            for t in &input {
                large_input.insert(
                    (t.0.0 + (max.0 + 1) * i, t.0.1 + (max.1 + 1) * j),
                    (t.1 + i as u32 + j as u32 - 1) % 9 + 1,
                );
            }
        }
    }

    (solve(&input), solve(&large_input))
}
