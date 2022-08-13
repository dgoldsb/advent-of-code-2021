extern crate lazy_static;

use aoc::parse_chars;
use core::cmp::max;
use core::cmp::min;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

lazy_static! {
    static ref UNSTABLE: HashSet<(usize, usize)> = vec![(3, 1), (5, 1), (7, 1), (9, 1)].into_iter().collect();
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    // Easy way to make a max-heap a min-heap: we go with negative scores.
    cost: isize,

    // The map.
    // TODO: Extract this, can be derived.
    available: HashSet<(usize, usize)>,

    // The amphipods.
    // TODO: Split these, each amphipod is their own being.
    amber: HashSet<(usize, usize)>,
    bronze: HashSet<(usize, usize)>,
    copper: HashSet<(usize, usize)>,
    desert: HashSet<(usize, usize)>,
}

// TODO: Get hash for free.
impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hashable_state = (
            (min(), max()),
            (min(), max()),
            (min(), max()),
            (min(), max()),
        );
        self.id.hash(hashable_state);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cartesian_distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    (max(start.0, end.0) - min(start.0, end.0)) + (max(start.1, end.1) - min(start.1, end.1))
}

fn is_reachable(start: &(usize, usize), end: &(usize, usize), available: &HashSet<(usize, usize)>) -> bool {
    if start == end {
        return true;
    }

    // Recursive up/down and left/right from the lower of t.
    let mut new_start;
    if end.0 > start.0 {
        new_start = (start.0 + 1, start.1);
    } else if end.0 < start.0 {
        new_start = (start.0 - 1, start.1);
    } else {
        new_start = (1000, 1000);
    }

    let side;
    if available.contains(&new_start) {
        side = is_reachable(&new_start, end, available);
    } else {
        side = false;
    }

    if end.1 > start.1 {
        new_start = (start.0, start.1 + 1);
    } else if end.1 < start.1 {
        new_start = (start.0, start.1 - 1);
    } else {
        new_start = (1000, 1000);
    }

    let up_down;
    if available.contains(&new_start) {
        up_down = is_reachable(&new_start, end, available);
    } else {
        up_down = false;
    }

    up_down || side
}

impl State {
    fn finished(&self) -> bool {
        return self.amber.contains(&(3, 2))
            && self.amber.contains(&(3, 3))
            && self.bronze.contains(&(5, 2))
            && self.bronze.contains(&(5, 3))
            && self.copper.contains(&(7, 2))
            && self.copper.contains(&(7, 3))
            && self.desert.contains(&(9, 2))
            && self.desert.contains(&(9, 3));
    }

    fn next_states_per_cost(
        &self,
        amphipods: &HashSet<(usize, usize)>,
        cost_per_move: usize,
    ) -> Vec<State> {
        let mut output = Vec::new();

        let transfer_row = 1;
        let home_column = match cost_per_move {
            1 => 3,
            10 => 5,
            100 => 7,
            1000 => 9,
            _ => panic!("Unexpected cost"),
        };

        for amphipod in amphipods {
            // Continue if shrimp is in home column and the spot below is not taken.
            if amphipod.0 == home_column {
                if amphipod.1 == 3 {
                    // The deepest, we can continue for sure.
                }
                // It is the top position, so we need to check the one below.
                // TODO: Deeper is recursive.
                let deeper = (amphipod.0, amphipod.1 + 1);
                if self.amber.contains(&deeper) && cost_per_move == 1 {
                    continue;
                }
                if self.bronze.contains(&deeper) && cost_per_move == 10 {
                    continue;
                }
                if self.copper.contains(&deeper) && cost_per_move == 100 {
                    continue;
                }
                if self.desert.contains(&deeper) && cost_per_move == 1000 {
                    continue;
                }
            }

            for available_spot in &self.available {
                // If you go into a room, go all the way.
                let deeper = (available_spot.0, available_spot.1 + 1);
                if self.available.contains(&deeper) {
                    continue;
                }

                // Amphipods will never move from the hallway into a room unless
                // that room contains no amphipods which do not also have that
                // room as their own destination.
                if self.amber.contains(&deeper) && cost_per_move != 1 {
                    continue;
                }
                if self.bronze.contains(&deeper) && cost_per_move != 10 {
                    continue;
                }
                if self.copper.contains(&deeper) && cost_per_move != 100 {
                    continue;
                }
                if self.desert.contains(&deeper) && cost_per_move != 1000 {
                    continue;
                }

                // Continue if location is unstable.
                if UNSTABLE.contains(&available_spot) {
                    continue;
                }

                // Continue if location is in same row.
                // println!("{}", cost_per_move);
                // println!("{:?} {:?}", amphipod, available_spot);
                if amphipod.1 == available_spot.1 {
                    continue;
                }

                // Continue if location is not in home column or transfer row.
                if !(available_spot.1 == transfer_row || available_spot.0 == home_column) {
                    continue;
                }

                // Continue if not reachable.
                if !is_reachable(amphipod, &available_spot, &self.available) {
                    continue;
                }

                // Calculate the distance (and thus cost).
                let distance: usize = cartesian_distance(amphipod, &available_spot);

                // Create the new state.
                let mut new_amber = self.amber.clone();
                let mut new_bronze = self.bronze.clone();
                let mut new_copper = self.copper.clone();
                let mut new_desert = self.desert.clone();
                let mut new_available = self.available.clone();

                new_available.remove(&available_spot);
                new_available.insert(amphipod.clone());

                match cost_per_move {
                    1 => {
                        new_amber.remove(amphipod);
                        new_amber.insert(*available_spot);
                    },
                    10 => {
                        new_bronze.remove(amphipod);
                        new_bronze.insert(*available_spot);
                    },
                    100 => {
                        new_copper.remove(amphipod);
                        new_copper.insert(*available_spot);
                    },
                    1000 => {
                        new_desert.remove(amphipod);
                        new_desert.insert(*available_spot);
                    },
                    _ => panic!("Unexpected cost"),
                }

                // Clone the state, with the two locations switched.
                output.push(
                    State {
                        cost: self.cost - (distance * cost_per_move) as isize,
                        available: new_available,
                        amber: new_amber,
                        bronze: new_bronze,
                        copper: new_copper,
                        desert: new_desert,
                    }
                );
            }
        }
        output
    }

    fn next_states(self) -> Vec<State> {
        self.next_states_per_cost(&self.amber, 1)
            .into_iter()
            .chain(self.next_states_per_cost(&self.bronze, 10).into_iter())
            .chain(self.next_states_per_cost(&self.copper, 100).into_iter())
            .chain(self.next_states_per_cost(&self.desert, 1000).into_iter())
            .collect::<Vec<State>>()
    }
}

fn solve(input: State) -> (usize, usize) {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(input);
    let part_a;

    // TODO: We need to cache known states, remove duplicates.
    let mut seen_states = HashSet::new();

    loop {
        let current = heap.pop().unwrap();
        // // println!("{:?}", &current.amber);
        // // println!("{:?}", &current.bronze);
        // // println!("{:?}", &current.copper);
        // // println!("{:?}", &current.desert);
        // println!("{}", &current.cost);
        if current.finished() {
            part_a = (-1 * current.cost) as usize;
            break;
        }
        let next_states = current.next_states();
        // println!("{:?}", &next_states.len());
        // if &next_states.len() < &10 {
        //     panic!("foo");
        // }
        for next_state in next_states {
            if !seen_states.contains(&next_state) {
                heap.push(next_state);
                seen_states.insert(next_state);
            }
        }
    }
    (part_a, 0)
}

pub fn day_23() -> (usize, usize) {
    let input = parse_chars("day_23".to_string());

    let mut available = HashSet::new();
    let mut amber = HashSet::new();
    let mut bronze = HashSet::new();
    let mut copper = HashSet::new();
    let mut desert = HashSet::new();

    for (i, c) in input.iter().enumerate() {
        let xy = (i % 14, i / 14);

        match c {
            '#' => true,
            '.' => available.insert(xy),
            'A' => amber.insert(xy),
            'B' => bronze.insert(xy),
            'C' => copper.insert(xy),
            'D' => desert.insert(xy),
            _ => true,
        };
    }

    let state = State {
        cost: 0,
        available,
        amber,
        bronze,
        copper,
        desert,
    };
    solve(state)
}

#[cfg(test)]
mod tests {
    use crate::day_23::solve;
    use crate::day_23::State;

    #[test]
    fn example_case() {
        let state = State {
            cost: 0,
            available: vec![
                (9, 1),
                (6, 1),
                (4, 1),
                (11, 1),
                (2, 1),
                (8, 1),
                (10, 1),
                (5, 1),
                (3, 1),
                (7, 1),
                (1, 1),
            ]
            .into_iter()
            .collect(),
            amber: vec![(9, 3), (3, 3)].into_iter().collect(),
            bronze: vec![(7, 2), (3, 2)].into_iter().collect(),
            copper: vec![(5, 2), (7, 3)].into_iter().collect(),
            desert: vec![(9, 2), (5, 3)].into_iter().collect(),
        };
        assert_eq!(solve(state), (12521, 0));
    }

    #[test]
    fn simple_case() {
        let state = State {
            cost: 0,
            available: vec![
                (9, 1),
                (6, 1),
                (4, 1),
                (11, 1),
                (2, 1),
                (8, 1),
                (10, 1),
                (5, 1),
                (3, 1),
                (7, 1),
                (1, 1),
            ]
                .into_iter()
                .collect(),
            amber: vec![(3, 2), (3, 3)].into_iter().collect(),
            bronze: vec![(7, 2), (5, 3)].into_iter().collect(),
            copper: vec![(5, 2), (7, 3)].into_iter().collect(),
            desert: vec![(9, 2), (9, 3)].into_iter().collect(),
        };
        assert_eq!(solve(state), (460, 0));
    }

    #[test]
    fn simplest_case() {
        let state = State {
            cost: 0,
            available: vec![
                (9, 1),
                (6, 1),
                (4, 1),
                (11, 1),
                (2, 1),
                (8, 1),
                (10, 1),
                (5, 1),
                (3, 1),
                (7, 1),
                (1, 1),
            ]
            .into_iter()
            .collect(),
            amber: vec![(3, 2), (3, 3)].into_iter().collect(),
            bronze: vec![(5, 2), (5, 3)].into_iter().collect(),
            copper: vec![(7, 2), (7, 3)].into_iter().collect(),
            desert: vec![(9, 2), (9, 3)].into_iter().collect(),
        };
        assert_eq!(solve(state), (0, 0));
    }
}
