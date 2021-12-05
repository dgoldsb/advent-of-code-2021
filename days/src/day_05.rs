use aoc::ints_from_str;
use aoc::parse_lines;
use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;

// TODO: Oof, this is slow. Refactor.

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

        for x in cmp::min(x0, x1)..=cmp::max(x0, x1) {
            for y in cmp::min(y0, y1)..=cmp::max(y0, y1) {
                if x0 == x1 || y0 == y1 || x == y {
                    vents.insert(Vent { x, y });
                }
            }
        }

        let diagonal = !(x0 == x1 || y0 == y1);

        return Ok(VentRow { diagonal, vents });
    }
}

impl VentRow {
    fn overlaps(&self, other: &VentRow) -> HashSet<Vent> {
        self.vents
            .intersection(&other.vents)
            .map(|v| v.clone())
            .collect::<HashSet<Vent>>()
    }
}

fn part_a(raw_inputs: &Vec<String>) -> usize {
    let vent_rows: Vec<VentRow> = raw_inputs
        .iter()
        .map(|s| VentRow::from_str(s).unwrap())
        .collect();
    let mut set: HashSet<Vent> = HashSet::new();

    for this in &vent_rows {
        for other in &vent_rows {
            // Filter out diagonals.
            if (this != other) && !this.diagonal && !other.diagonal {
                let overlap = this.overlaps(other);
                set = set.union(&overlap).map(|v| v.clone()).collect();
            }
        }
    }
    set.len()
}

pub fn day_05() {
    let raw_inputs = parse_lines("day_05".to_string());
    println!("A: {}", part_a(&raw_inputs));
    println!("B: {}", part_a(&raw_inputs));
}
