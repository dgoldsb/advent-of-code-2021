use aoc::parse_ints;

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
    if part_a {
        let mut position_one = input.get(1).unwrap().clone() as usize;
        let mut position_two = input.get(3).unwrap().clone() as usize;
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
        0
    }
}

pub fn day_21() -> (usize, usize) {
    let input = parse_ints("day_21".to_string());
    (solve(&input, true), solve(&input, false))
}
