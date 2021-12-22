use aoc::ints_from_str;
use aoc::parse_lines;
use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
struct Cuboid {
    on: bool,
    xs: (isize, isize),
    ys: (isize, isize),
    zs: (isize, isize),
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(input: &str) -> Result<Cuboid, Self::Err> {
        let on = input.contains("on");
        let v = ints_from_str(&input.to_string());
        let mut is = v.iter();
        let xs = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        let ys = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        let zs = (is.next().unwrap().to_owned(), is.next().unwrap().to_owned());
        return Ok(Cuboid { on, xs, ys, zs });
    }
}

impl Add for Cuboid {
    type Output = Vec<Cuboid>;
    fn add(self, other: Cuboid) -> Vec<Cuboid> {
        // This add is symmetrical, meaning it is important for the problem that self is never off.
        if !self.on {
            panic!("this is not according to the plan, required assumption")
        }

        // Return vector.
        let mut result = Vec::new();

        // Calculate all cubes that remain when we overlap the two.
        if self.contains(&other) {
            result.push(self);
        } else if other.contains(&self) {
            result.push(other)
        } else if !self.intersects(other) && ! other.intersects(self) {
            result.push(self);
            result.push(other);
        } else {
            // TODO.
        }

        // We do not yield the resulting cuboids that are off, filter them out.
        result.iter().filter(|c| **c.on).map(|c| *c).collect::<Vec<Cuboid>>()
    }
}

impl Cuboid {
    fn contains(self, other: &Cuboid) -> bool {
        let rx = (self.xs.0..=self.xs.1);
        let ry = (self.ys.0..=self.ys.1);
        let rz = (self.zs.0..=self.zs.1);

        return rx.contains(other.xs.0)
            && rx.contains(other.xs.1)
            && ry.contains(other.ys.0)
            && ry.contains(other.ys.1)
            && rz.contains(other.zs.0)
            && rz.contains(other.zs.1);
    }

    fn intersects(self, other: &Cuboid) -> bool {
        if self.contains(other) {
            return false;
        }

        let rx = (self.xs.0..=self.xs.1);
        let ry = (self.ys.0..=self.ys.1);
        let rz = (self.zs.0..=self.zs.1);

        return (rx.contains(other.xs.0) || rx.contains(other.xs.1))
            && (ry.contains(other.ys.0) || ry.contains(other.ys.1))
            && (rz.contains(other.zs.0) || rz.contains(other.zs.1));
    }

    fn is_valid(self) -> bool {
        (self.xs.0 < self.xs.1) && (self.ys.0 < self.ys.1) && (self.zs.0 < self.zs.1)
    }
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    let mut set_a: HashSet<(isize, isize, isize)> = HashSet::new();

    for step in input.iter().map(|s| Cuboid::from_str(s).unwrap()) {
        // Part a, slow approach.
        for x in step.xs.0..=step.xs.1 {
            let part_a = x.abs() <= 50;
            if !part_a {
                break;
            }
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
