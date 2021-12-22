use aoc::parse_ints;
use std::cmp::max;
use std::collections::HashMap;

struct DeterministicDie {
    next_value: usize,
}

impl Iterator for DeterministicDie {
    type Item = (usize, usize, usize);

    fn next(&mut self) -> Option<(usize, usize, usize)> {
        let x = self.next_value + 1;
        self.next_value = (self.next_value + 1) % 100;
        let y = self.next_value + 1;
        self.next_value = (self.next_value + 1) % 100;
        let z = self.next_value + 1;
        self.next_value = (self.next_value + 1) % 100;
        return Option::Some((x, y, z));
    }
}

impl Default for DeterministicDie {
    fn default() -> Self {
        DeterministicDie { next_value: 0 }
    }
}

fn solve(input: &Vec<isize>, part_a: bool) -> usize {
    let p1 = input.get(1).unwrap().clone() as usize;
    let p2 = input.get(3).unwrap().clone() as usize;
    if part_a {
        let mut position_one = p1;
        let mut position_two = p2;
        let mut score_one = 0;
        let mut score_two = 0;
        for (i, x) in DeterministicDie::default().enumerate() {
            if i % 2 == 0 {
                position_one = (position_one - 1 + x.0 + x.1 + x.2) % 10 + 1;
                score_one += position_one;
            } else {
                position_two = (position_two - 1 + x.0 + x.1 + x.2) % 10 + 1;
                score_two += position_two;
            }

            if score_two >= 1000 {
                return score_one * ((i + 1) * 3);
            } else if score_one >= 1000 {
                return score_two * ((i + 1) * 3);
            }
        }
        panic!("the die broke")
    } else {
        let mut state_count: HashMap<(usize, usize, usize, usize), usize> = HashMap::new();
        state_count.insert((p1, p2, 0, 0), 1);

        let mut p1_win = 0;
        let mut p2_win = 0;

        let default = 0;
        for i in 0..1000 {
            // Evolve.
            let mut new_map: HashMap<(usize, usize, usize, usize), usize> = HashMap::new();
            for d1 in 1..=3 {
                for d2 in 1..=3 {
                    for d3 in 1..=3 {
                        for (key, value) in &state_count {
                            let new_key;
                            if i % 2 == 0 {
                                let position_one = (key.0 - 1 + d1 + d2 + d3) % 10 + 1;
                                let score_one = key.2 + position_one;

                                new_key = (position_one, key.1, score_one, key.3);
                            } else {
                                let position_two = (key.1 - 1 + d1 + d2 + d3) % 10 + 1;
                                let score_two = key.3 + position_two;
                                new_key = (key.0, position_two, key.2, score_two);
                            }

                            if new_map.contains_key(&new_key) {
                                let mut_value = new_map.get_mut(&new_key).unwrap();
                                *mut_value += value;
                            } else {
                                new_map.insert(new_key, value.clone());
                            }
                        }
                    }
                }
            }

            // Count wins.
            let mut new_new_map: HashMap<(usize, usize, usize, usize), usize> = HashMap::new();
            for (key, value) in new_map {
                if key.2 >= 21 {
                    p1_win += value;
                } else if key.3 >= 21 {
                    p2_win += value;
                } else {
                    new_new_map.insert(key, value);
                }
            }

            // Finish iteration.
            state_count = new_new_map;
        }
        max(p1_win, p2_win)
    }
}

pub fn day_21() -> (usize, usize) {
    let input = parse_ints("day_21".to_string());
    (solve(&input, true), solve(&input, false))
}
