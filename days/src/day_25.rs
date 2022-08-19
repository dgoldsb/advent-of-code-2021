use aoc::parse_lines;
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
struct State {
    east: HashSet<(isize, isize)>,
    south: HashSet<(isize, isize)>,
    max_x: isize,
    max_y: isize,
}

impl State {
    fn advance(&self) -> Result<State, &str> {
        let mut east: HashSet<(isize, isize)> = HashSet::new();
        let mut south: HashSet<(isize, isize)> = HashSet::new();

        for et in &self.east {
            let new_et = ((et.0 + 1) % self.max_x, et.1);
            if self.east.contains(&new_et) || self.south.contains(&new_et) {
                east.insert(*et);
            } else {
                east.insert(new_et);
            }
        }

        for st in &self.south {
            let new_st = (st.0, -1 * (((-1 * st.1) + 1) % self.max_y));
            if self.south.contains(&new_st) || east.contains(&new_st) {
                south.insert(*st);
            } else {
                south.insert(new_st);
            }
        }

        let new_state = State {
            east,
            south,
            max_x: self.max_x,
            max_y: self.max_y,
        };

        if self == &new_state {
            Err("No change in state")
        } else {
            Ok(new_state)
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for y in 0..self.max_y {
            let mut line = String::new();
            for x in 0..self.max_x {
                let tuple = (x, -1 * y);
                if self.south.contains(&tuple) {
                    line.push('v');
                } else if self.east.contains(&tuple) {
                    line.push('>');
                } else {
                    line.push('.');
                }
            }
            line.push('\n');
            output.push_str(&line);
        }
        output
    }
}

fn solve() -> usize {
    let mut counter: usize = 0;
    let mut state = get_state();

    loop {
        counter += 1;
        match state.advance() {
            Ok(s) => {
                state = s;
            }
            Err(_) => {
                return counter;
            }
        }
    }
}

fn get_state() -> State {
    let mut east = HashSet::new();
    let mut south = HashSet::new();
    for (i, line) in parse_lines("day_25".to_string()).iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '>' => east.insert((j as isize, -1 * (i as isize))),
                'v' => south.insert((j as isize, -1 * (i as isize))),
                _ => false,
            };
        }
    }

    let max_x = east.iter().chain(south.iter()).map(|t| t.0).max().unwrap() + 1;
    let max_y = east
        .iter()
        .chain(south.iter())
        .map(|t| t.1 * -1)
        .max()
        .unwrap()
        + 1;

    State {
        east,
        south,
        max_x,
        max_y,
    }
}

pub fn day_25() -> (usize, usize) {
    (solve(), 0)
}
