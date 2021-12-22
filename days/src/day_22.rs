use std::collections::HashSet;
use aoc::parse_lines;
use aoc::ints_from_str;
use std::str::FromStr;

#[derive(Debug)]
struct RebootStep {
    on: bool,
    xs: (isize, isize),
    ys: (isize, isize),
    zs: (isize, isize),
}

impl FromStr for RebootStep {
    type Err = ();

    fn from_str(input: &str) -> Result<RebootStep, Self::Err> {
        let on = input.contains("on");
        let v = ints_from_str(&input.to_string());
        let mut is = v.iter();
        let xs = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        let ys = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        let zs = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        return Ok(RebootStep { on, xs, ys, zs })
    }
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    let mut set_a: HashSet<(isize, isize, isize)> = HashSet::new();

    for step in input.iter().map(|s| RebootStep::from_str(s).unwrap()) {
        // Part a, slow approach.
        for x in step.xs.0..=step.xs.1 {
            let part_a = x.abs() <= 50;
            if !part_a {break;}
            for y in step.ys.0..=step.ys.1 {
                for z in step.zs.0..=step.zs.1 {
                    let v = (x, y, z);
                    if step.on {
                        set_a.insert(v);
                    } else {
                        set_a.remove(&v);
                    }
                }
            }
        }

        // Part b.
    }
    (set_a.len(), 0)
}

pub fn day_22() -> (usize, usize) {
    let input = parse_lines("day_22".to_string());
    solve(&input)
}
