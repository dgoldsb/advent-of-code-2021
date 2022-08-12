use aoc::ints_from_str;
use aoc::parse_lines;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

impl Cuboid {
    fn volume(self) -> usize {
        return ((self.xs.1 - self.xs.0 + 1)
            * (self.zs.1 - self.zs.0 + 1)
            * (self.ys.1 - self.ys.0 + 1)) as usize;
    }

    fn limited_volume(self) -> usize {
        return ((min(50, self.xs.1) - max(-50, self.xs.0) + 1)
            * (min(50, self.zs.1) - max(-50, self.zs.0) + 1)
            * (min(50, self.ys.1) - max(-50, self.ys.0) + 1)) as usize;
    }

    fn contains(self, other: &Cuboid) -> bool {
        let rx = self.xs.0..=self.xs.1;
        let ry = self.ys.0..=self.ys.1;
        let rz = self.zs.0..=self.zs.1;

        return rx.contains(&other.xs.0)
            && rx.contains(&other.xs.1)
            && ry.contains(&other.ys.0)
            && ry.contains(&other.ys.1)
            && rz.contains(&other.zs.0)
            && rz.contains(&other.zs.1);
    }

    fn intersects(self, other: &Cuboid) -> bool {
        if self.contains(other) || other.contains(&self) {
            return false;
        }

        let rx = self.xs.0..=self.xs.1;
        let ry = self.ys.0..=self.ys.1;
        let rz = self.zs.0..=self.zs.1;

        return (rx.contains(&other.xs.0) || rx.contains(&other.xs.1))
            && (ry.contains(&other.ys.0) || ry.contains(&other.ys.1))
            && (rz.contains(&other.zs.0) || rz.contains(&other.zs.1));
    }

    fn is_valid(self) -> bool {
        (self.xs.0 <= self.xs.1) && (self.ys.0 <= self.ys.1) && (self.zs.0 <= self.zs.1)
    }

    fn intersection(self, other: &Cuboid) -> Cuboid {
        if !self.intersects(other) {
            panic!("This does not intersect")
        }

        return Cuboid {
            on: self.on,
            xs: (max(self.xs.0, other.xs.0), min(self.xs.1, other.xs.1)),
            ys: (max(self.ys.0, other.ys.0), min(self.ys.1, other.ys.1)),
            zs: (max(self.zs.0, other.zs.0), min(self.zs.1, other.zs.1)),
        };
    }

    // Get up to 7 cuboids, minus the inner, so 6.
    fn get_many_cuboids(self, inner: &Cuboid) -> Vec<Cuboid> {
        let mut output: Vec<Cuboid> = Vec::new();

        // Left plate.
        output.push(Cuboid {
            on: self.on,
            xs: (self.xs.0, inner.xs.0 - 1),
            ys: (self.ys.0, self.ys.1),
            zs: (self.zs.0, self.zs.1),
        });
        // Right plate.
        output.push(Cuboid {
            on: self.on,
            xs: (inner.xs.1 + 1, self.xs.1),
            ys: (self.ys.0, self.ys.1),
            zs: (self.zs.0, self.zs.1),
        });
        // Front pillar.
        output.push(Cuboid {
            on: self.on,
            xs: (inner.xs.0, inner.xs.1),
            ys: (self.ys.0, inner.ys.0 - 1),
            zs: (self.zs.0, self.zs.1),
        });
        // Back pillar
        output.push(Cuboid {
            on: self.on,
            xs: (inner.xs.0, inner.xs.1),
            ys: (inner.ys.1 + 1, self.ys.1),
            zs: (self.zs.0, self.zs.1),
        });
        // Bottom slab.
        output.push(Cuboid {
            on: self.on,
            xs: (inner.xs.0, inner.xs.1),
            ys: (inner.ys.0, inner.ys.1),
            zs: (self.zs.0, inner.zs.0 - 1),
        });
        // Upper slab.
        output.push(Cuboid {
            on: self.on,
            xs: (inner.xs.0, inner.xs.1),
            ys: (inner.ys.0, inner.ys.1),
            zs: (inner.zs.1 + 1, self.zs.1),
        });

        output
            .iter()
            .filter(|&&c| c.is_valid())
            .map(|&c| c)
            .collect()
    }

    // Shatter the other.
    fn shatter(self, other: &Cuboid) -> Vec<Cuboid> {
        // Find the joint cuboid.
        let joint_cuboid = self.intersection(other);

        // Find the 8 recessive cuboids.
        other.get_many_cuboids(&joint_cuboid)
    }
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    let mut cuboid_set: HashSet<Cuboid> = HashSet::new();

    for new_cuboid in input.iter().map(|s| Cuboid::from_str(s).unwrap()) {
        let mut new_cuboids = HashSet::new();
        for cuboid in cuboid_set {
            if new_cuboid.intersects(&cuboid) {
                for shard in new_cuboid.shatter(&cuboid) {
                    new_cuboids.insert(shard);
                }
            } else {
                new_cuboids.insert(cuboid);
            }
        }
        new_cuboids.insert(new_cuboid);

        let active_cuboids = new_cuboids.iter().filter(|c| c.on).map(|c| *c).collect();

        cuboid_set = active_cuboids;
    }
    (
        cuboid_set.iter().map(|c| c.limited_volume()).sum(),
        cuboid_set.iter().map(|c| c.volume()).sum(),
    )
}

pub fn day_22() -> (usize, usize) {
    let input = parse_lines("day_22_test".to_string());
    solve(&input)
}


#[cfg(test)]
mod tests {
    use crate::day_22::solve;

    #[test]
    fn simplest_case() {
        let input = vec![
            "on x=10..12,y=10..12,z=10..12".to_string(),
            "on x=11..13,y=11..13,z=11..13".to_string(),
            "off x=9..11,y=9..11,z=9..11".to_string(),
            "on x=10..10,y=10..10,z=10..10".to_string(),
        ];
        assert_eq!(solve(&input), (39, 39));
    }

    #[test]
    fn simple_case() {
        let input = vec![
            "on x=-20..33,y=-21..23,z=-26..28".to_string(),
            "on x=-22..28,y=-29..23,z=-38..16".to_string(),
            "on x=-46..7,y=-6..46,z=-50..-1".to_string(),
            "on x=-49..1,y=-3..46,z=-24..28".to_string(),
            "on x=2..47,y=-22..22,z=-23..27".to_string(),
            "on x=-27..23,y=-28..26,z=-21..29".to_string(),
            "on x=-39..5,y=-6..47,z=-3..44".to_string(),
            "on x=-30..21,y=-8..43,z=-13..34".to_string(),
            "on x=-22..26,y=-27..20,z=-29..19".to_string(),
            "off x=-48..-32,y=26..41,z=-47..-37".to_string(),
            "on x=-12..35,y=6..50,z=-50..-2".to_string(),
            "off x=-48..-32,y=-32..-16,z=-15..-5".to_string(),
            "on x=-18..26,y=-33..15,z=-7..46".to_string(),
            "off x=-40..-22,y=-38..-28,z=23..41".to_string(),
            "on x=-16..35,y=-41..10,z=-47..6".to_string(),
            "off x=-32..-23,y=11..30,z=-14..3".to_string(),
            "on x=-49..-5,y=-3..45,z=-29..18".to_string(),
            "off x=18..30,y=-20..-8,z=-3..13".to_string(),
            "on x=-41..9,y=-7..43,z=-33..15".to_string(),
        ];
        assert_eq!(solve(&input).0, 590784);
    }
}

