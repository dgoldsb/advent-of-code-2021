extern crate lazy_static;

use aoc::manhattan_distance;
use aoc::parse_chars;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

const TRANSFER_ROW: usize = 1;

lazy_static! {
    static ref UNSTABLE: HashSet<(usize, usize)> =
        vec![(3, 1), (5, 1), (7, 1), (9, 1)].into_iter().collect();
}

lazy_static! {
    static ref POSSIBLE: HashSet<(usize, usize)> = vec![
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
        (3, 2),
        (3, 3),
        (5, 2),
        (5, 3),
        (7, 2),
        (7, 3),
        (9, 2),
        (9, 3)
    ]
    .into_iter()
    .collect();
}

lazy_static! {
    static ref POSSIBLE_B: HashSet<(usize, usize)> = vec![
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
        (3, 2),
        (3, 3),
        (5, 2),
        (5, 3),
        (7, 2),
        (7, 3),
        (9, 2),
        (9, 3),
        (3, 4),
        (3, 5),
        (5, 4),
        (5, 5),
        (7, 4),
        (7, 5),
        (9, 4),
        (9, 5),
    ]
    .into_iter()
    .collect();
}

fn is_reachable(
    start: &(usize, usize),
    end: &(usize, usize),
    available: &HashSet<(usize, usize)>,
) -> bool {
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

#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct Amphipod {
    location: (usize, usize),
    cost_per_move: isize,
}

impl Amphipod {
    fn home_column(&self) -> usize {
        match self.cost_per_move {
            1 => 3,
            10 => 5,
            100 => 7,
            1000 => 9,
            _ => panic!("Unexpected cost"),
        }
    }

    fn is_home(&self) -> bool {
        return self.location.0 == self.home_column();
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    // Easy way to make a max-heap a min-heap: we go with negative scores.
    cost: isize,
    // A state consists of 8-16 amphipodes.
    amphipodes: Vec<Amphipod>,
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

impl State {
    fn finished(&self) -> bool {
        self.amphipodes.iter().all(|a| a.is_home())
    }

    fn taken(&self) -> HashSet<(usize, usize)> {
        self.amphipodes.iter().map(|a| a.location).collect()
    }

    fn available(&self) -> HashSet<(usize, usize)> {
        let taken = &self.taken();
        let set = if self.amphipodes.len() == 8 {
            POSSIBLE.difference(taken)
        } else {
            POSSIBLE_B.difference(taken)
        };

        set.map(|l| l.clone()).collect::<HashSet<(usize, usize)>>()
    }

    // TODO: reuse in next?
    fn get_deepest(
        &self,
        available: &HashSet<(usize, usize)>,
        current: &(usize, usize),
    ) -> (usize, usize) {
        let deeper = (current.0, current.1 + 1);

        return if available.contains(&deeper) {
            self.get_deepest(available, &deeper)
        } else {
            *current
        };
    }

    fn deeper_amphipodes_are_of_same_type(
        &self,
        original: &Amphipod,
        current: &(usize, usize),
    ) -> bool {
        //todo
        let deeper = (current.0, current.1 + 1);

        let this_okay = self
            .amphipodes
            .iter()
            .filter(|a| a.location == deeper)
            .filter(|a| a.cost_per_move != original.cost_per_move)
            .collect::<Vec<&Amphipod>>()
            .len()
            == 0;

        let contains = if self.amphipodes.len() == 8 {
            POSSIBLE.contains(&deeper)
        } else {
            POSSIBLE_B.contains(&deeper)
        };

        return if contains {
            self.deeper_amphipodes_are_of_same_type(original, &deeper) && this_okay
        } else {
            this_okay
        };
    }

    fn amphipod_is_home(&self, amphipod: Amphipod) -> bool {
        amphipod.is_home() && self.deeper_amphipodes_are_of_same_type(&amphipod, &amphipod.location)
    }

    fn is_valid_move(&self, amphipod: Amphipod, target: &(usize, usize)) -> bool {
        // The amphipod cannot move deeper.
        (target == &self.get_deepest(&self.available(), target)) &&
        // Amphipods will never move from the hallway into a room unless
        // that room contains no amphipods which do not also have that
        // room as their own destination.
        self.deeper_amphipodes_are_of_same_type(&amphipod, target) &&
        // The location is not unstable.
        !UNSTABLE.contains(target) &&
        // The location is not in same row.
        (amphipod.location.1 != target.1) &&
        // Target location is transfer row or home column.
        (target.1 == TRANSFER_ROW || target.0 == amphipod.home_column()) &&
        // The location is reachable.
        is_reachable(&amphipod.location, target, &self.available())
    }

    fn next_states_for_amphipod(&self, amphipod: Amphipod) -> Vec<State> {
        let mut output = Vec::new();

        // Continue if shrimp is in home column and the spot below is not taken.
        if self.amphipod_is_home(amphipod) {
            return output;
        }

        for available_spot in self.available() {
            if !self.is_valid_move(amphipod, &available_spot) {
                continue;
            }

            // Create the new state.
            let distance = manhattan_distance(&amphipod.location, &available_spot);
            let mut new_amphipodes: Vec<Amphipod> = self
                .amphipodes
                .iter()
                .filter(|&a| a != &amphipod)
                .map(|a| *a)
                .collect();
            new_amphipodes.push(Amphipod {
                cost_per_move: amphipod.cost_per_move,
                location: available_spot,
            });
            new_amphipodes.sort();
            output.push(State {
                cost: self.cost - (distance as isize * amphipod.cost_per_move),
                amphipodes: new_amphipodes,
            });
        }
        return output;
    }

    fn next_states(self) -> Vec<State> {
        self.amphipodes
            .iter()
            .map(|a| self.next_states_for_amphipod(*a))
            .flatten()
            .collect::<Vec<State>>()
    }
}

fn solve(input: State) -> usize {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(input);
    let part_a;

    // We need to cache known states, remove duplicates.
    // Assumption: the first time we encounter a state is the lowest energy.
    let mut seen_states = HashSet::new();

    loop {
        let current = heap.pop().unwrap();
        if current.finished() {
            part_a = (-1 * current.cost) as usize;
            break;
        }
        let next_states = current.next_states();
        for next_state in next_states {
            if !seen_states.contains(&next_state) {
                heap.push(next_state.clone());
                seen_states.insert(next_state);
            }
        }
    }
    part_a
}

fn parse_input(input: Vec<char>) -> State {
    let mut amphipodes = Vec::new();

    let mut i = 0;
    for c in input {
        let cost_per_move = match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        };
        if cost_per_move > 0 {
            let location = (
                match i % 4 {
                    0 => 3,
                    1 => 5,
                    2 => 7,
                    3 => 9,
                    _ => panic!(),
                },
                2 + i / 4,
            );
            i = i + 1;
            amphipodes.push(Amphipod {
                cost_per_move,
                location,
            });
        }
    }
    State {
        cost: 0,
        amphipodes,
    }
}

pub fn day_23() -> (usize, usize) {
    // TODO: Remove second input file.
    let input_a = parse_chars("day_23_a".to_string());
    let input_b = parse_chars("day_23_b".to_string());

    (solve(parse_input(input_a)), solve(parse_input(input_b)))
}

#[cfg(test)]
mod tests {
    use crate::day_23::solve;
    use crate::day_23::Amphipod;
    use crate::day_23::State;

    #[test]
    fn example_case() {
        let state = State {
            cost: 0,
            amphipodes: vec![
                Amphipod {
                    cost_per_move: 1,
                    location: (9, 3),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 3),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (7, 2),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (3, 2),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (5, 2),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 3),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 2),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (5, 3),
                },
            ],
        };
        assert_eq!(solve(state), 12521);
    }

    #[test]
    fn simple_case() {
        let state = State {
            cost: 0,
            amphipodes: vec![
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 2),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 3),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (7, 2),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 3),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (5, 2),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 3),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 2),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 3),
                },
            ],
        };
        assert_eq!(solve(state), 460);
    }

    #[test]
    fn simple_case_b() {
        let state = State {
            cost: 0,
            amphipodes: vec![
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 2),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 3),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 4),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 5),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (7, 2),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 3),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 4),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 5),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (5, 2),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 3),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 4),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 5),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 2),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 3),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 4),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 5),
                },
            ],
        };
        assert_eq!(solve(state), 460);
    }

    #[test]
    fn simplest_case() {
        let state = State {
            cost: 0,
            amphipodes: vec![
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 2),
                },
                Amphipod {
                    cost_per_move: 1,
                    location: (3, 3),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 2),
                },
                Amphipod {
                    cost_per_move: 10,
                    location: (5, 3),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 2),
                },
                Amphipod {
                    cost_per_move: 100,
                    location: (7, 3),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 2),
                },
                Amphipod {
                    cost_per_move: 1000,
                    location: (9, 3),
                },
            ],
        };
        assert_eq!(solve(state), 0);
    }
}
