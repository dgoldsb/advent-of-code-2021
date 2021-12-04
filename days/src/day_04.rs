use aoc::ints_from_str;
use aoc::parse_items;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct BingoBoard {
    rows: Vec<HashSet<isize>>,
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(input: &str) -> Result<BingoBoard, Self::Err> {
        let dimension = 5;
        let numbers = ints_from_str(&input.to_string());
        let mut rows = Vec::new();

        // Get the horizonal rows.
        for i in 0..dimension {
            let set = numbers[i..(i + dimension)]
                .iter()
                .map(|i| *i)
                .collect::<HashSet<isize>>();
            rows.push(set);
        }

        // Get the vertical rows.
        for i in 0..dimension {
            let mut set = HashSet::new();
            for j in 0..dimension {
                set.insert(*numbers.get(i + j * dimension).unwrap());
            }
            rows.push(set);
        }

        return Ok(BingoBoard { rows });
    }
}

impl BingoBoard {
    fn check(&self, drawn: &HashSet<isize>) -> bool {
        for row in &self.rows {
            if row.is_subset(drawn) {
                return true;
            }
        }
        return false;
    }

    fn score(&self, drawn: &HashSet<isize>) -> isize {
        let all = self
            .rows
            .iter()
            .fold(HashSet::new(), |i, j| i.union(&j).map(|i| *i).collect());
        let some = all.difference(drawn);
        some.sum()
    }
}

fn part_a(raw_inputs: &Vec<String>) -> isize {
    let numbers_to_draw: Vec<isize> = ints_from_str(raw_inputs.get(0).unwrap());
    let slice = &raw_inputs[1..raw_inputs.len()];
    let boards: Vec<BingoBoard> = slice
        .iter()
        .map(|s| BingoBoard::from_str(s).unwrap())
        .collect();

    let mut drawn_numbers = HashSet::new();
    for drawn_number in numbers_to_draw {
        drawn_numbers.insert(drawn_number);
        for board in &boards {
            if board.check(&drawn_numbers) {
                return drawn_number * board.score(&drawn_numbers);
            }
        }
    }
    panic!("no solution found")
}

pub fn day_04() {
    let instructions = parse_items("day_04".to_string(), "\n\n".to_string());
    println!("A: {}", part_a(&instructions));
    println!("B: {}", part_a(&instructions));
}
