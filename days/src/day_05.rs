use aoc::ints_from_str;
use aoc::parse_lines;
use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vent {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VentRow {
    diagonal: bool,
    vents: HashSet<Vent>,
}

impl FromStr for VentRow {
    type Err = ();

    fn from_str(input: &str) -> Result<VentRow, Self::Err> {
        let numbers = ints_from_str(&input.to_string());
        let mut vents = HashSet::new();

        let x0 = *numbers.get(0).unwrap();
        let x1 = *numbers.get(2).unwrap();
        let y0 = *numbers.get(1).unwrap();
        let y1 = *numbers.get(3).unwrap();

        // Horizontal and vertical.
        for x in cmp::min(x0, x1)..=cmp::max(x0, x1) {
            for y in cmp::min(y0, y1)..=cmp::max(y0, y1) {
                if x0 == x1 || y0 == y1 {
                    vents.insert(Vent { x, y });
                }
            }
        }

        // Diagonal, less elegant because Rust does not appear to like descending ranges,
        let mut x = x0;
        let mut y = y0;
        while (x != x1) && (y != y1) {
            vents.insert(Vent { x, y });
            if x0 != x1 {
                x += (x1 - x0) / (x1 - x0).abs();
            }
            if y0 != y1 {
                y += (y1 - y0) / (y1 - y0).abs();
            }

            if (x == x1) && (y == y1) {
                vents.insert(Vent { x, y });
            }
        }

        let diagonal = !(x0 == x1 || y0 == y1);

        return Ok(VentRow { diagonal, vents });
    }
}

impl VentRow {
    fn overlaps(&self, other: &HashSet<Vent>) -> HashSet<Vent> {
        self.vents
            .intersection(&other)
            .map(|v| v.clone())
            .collect::<HashSet<Vent>>()
    }
}

fn solve(raw_inputs: &Vec<String>, diagonals: bool) -> usize {
    let vent_rows: Vec<VentRow> = raw_inputs
        .iter()
        .map(|s| VentRow::from_str(s).unwrap())
        .collect();

    let mut taken: HashSet<Vent> = HashSet::new();
    let mut overlaps: HashSet<Vent> = HashSet::new();

    for vent_row in &vent_rows {
        // Filter out diagonals if necessary.
        if !vent_row.diagonal || diagonals {
            overlaps = overlaps
                .union(&vent_row.overlaps(&taken))
                .map(|v| v.clone())
                .collect();
            taken = taken.union(&vent_row.vents).map(|v| v.clone()).collect();
        }
    }

    overlaps.len()
}

pub fn day_05() {
    let raw_inputs = parse_lines("day_05".to_string());
    println!("A: {}", solve(&raw_inputs, false));
    println!("B: {}", solve(&raw_inputs, true));
}
