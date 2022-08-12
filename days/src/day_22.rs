// Disclaimer: this one got messy, I decided to go for a range but that came back to
// haunt me as it does not implement the Copy trait, resulting in excessive cloning.
use aoc::ints_from_str;
use aoc::parse_lines;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Cuboid {
    on: bool,
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    zs: RangeInclusive<isize>,
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(input: &str) -> Result<Cuboid, Self::Err> {
        let on = input.contains("on");
        let v = ints_from_str(&input.to_string());
        let mut is = v.iter();
        let xs = is.next().unwrap().to_owned()..=is.next().unwrap().to_owned();
        let ys = is.next().unwrap().to_owned()..=is.next().unwrap().to_owned();
        let zs = is.next().unwrap().to_owned()..=is.next().unwrap().to_owned();
        return Ok(Cuboid { on, xs, ys, zs });
    }
}

impl Clone for Cuboid {
    fn clone(&self) -> Cuboid {
        Cuboid {
            on: self.on,
            xs: *self.xs.start()..=*self.xs.end(),
            ys: *self.ys.start()..=*self.ys.end(),
            zs: *self.zs.start()..=*self.zs.end(),
        }
    }
}

impl Cuboid {
    fn volume(self) -> usize {
        ((self.xs.end() - self.xs.start() + 1)
            * (self.ys.end() - self.ys.start() + 1)
            * (self.zs.end() - self.zs.start() + 1)) as usize
    }

    fn limited_volume(self) -> usize {
        let cuboid = Cuboid {
            on: self.on,
            xs: max(-50, *self.xs.start())..=min(50, *self.xs.end()),
            ys: max(-50, *self.ys.start())..=min(50, *self.ys.end()),
            zs: max(-50, *self.zs.start())..=min(50, *self.zs.end()),
        };
        if cuboid.clone().is_valid() {
            return cuboid.volume();
        } else {
            return 0;
        }
    }

    fn is_valid(self) -> bool {
        (*self.xs.start() <= *self.xs.end())
            && (*self.ys.start() <= *self.ys.end())
            && (*self.zs.start() <= *self.zs.end())
    }

    fn intersection(self, other: &Cuboid) -> Cuboid {
        return Cuboid {
            on: self.on,
            xs: max(*self.xs.start(), *other.xs.start())..=min(*self.xs.end(), *other.xs.end()),
            ys: max(*self.ys.start(), *other.ys.start())..=min(*self.ys.end(), *other.ys.end()),
            zs: max(*self.zs.start(), *other.zs.start())..=min(*self.zs.end(), *other.zs.end()),
        };
    }

    // Get up to 7 cuboids.
    fn get_many_cuboids(self, inner: &Cuboid) -> Vec<Cuboid> {
        let mut output: Vec<Cuboid> = Vec::new();

        // Left plate.
        output.push(Cuboid {
            on: self.on,
            xs: *self.xs.start()..=(*inner.xs.start() - 1),
            ys: *self.ys.start()..=*self.ys.end(),
            zs: *self.zs.start()..=*self.zs.end(),
        });
        // Right plate.
        output.push(Cuboid {
            on: self.on,
            xs: (*inner.xs.end() + 1)..=*self.xs.end(),
            ys: *self.ys.start()..=*self.ys.end(),
            zs: *self.zs.start()..=*self.zs.end(),
        });
        // Front pillar.
        output.push(Cuboid {
            on: self.on,
            xs: *inner.xs.start()..=*inner.xs.end(),
            ys: *self.ys.start()..=(*inner.ys.start() - 1),
            zs: *self.zs.start()..=*self.zs.end(),
        });
        // Back pillar
        output.push(Cuboid {
            on: self.on,
            xs: *inner.xs.start()..=*inner.xs.end(),
            ys: (inner.ys.end() + 1)..=*self.ys.end(),
            zs: *self.zs.start()..=*self.zs.end(),
        });
        // Bottom slab.
        output.push(Cuboid {
            on: self.on,
            xs: *inner.xs.start()..=*inner.xs.end(),
            ys: *inner.ys.start()..=*inner.ys.end(),
            zs: *self.zs.start()..=(*inner.zs.start() - 1),
        });
        // Upper slab.
        output.push(Cuboid {
            on: self.on,
            xs: *inner.xs.start()..=*inner.xs.end(),
            ys: *inner.ys.start()..=*inner.ys.end(),
            zs: (*inner.zs.end() + 1)..=*self.zs.end(),
        });

        output
            .iter()
            .map(|c| c.clone())
            .filter(|c| c.clone().is_valid())
            .collect()
    }

    // Shatter the other.
    fn shatter(self, other: &Cuboid) -> Vec<Cuboid> {
        // Find the joint cuboid.
        let joint_cuboid = self.intersection(other);

        if !joint_cuboid.clone().is_valid() {
            return vec![other.clone()];
        }

        other.clone().get_many_cuboids(&joint_cuboid)
    }
}

fn solve(input: &Vec<String>) -> (usize, usize) {
    let mut cuboid_set: HashSet<Cuboid> = HashSet::new();

    for new_cuboid in input.iter().map(|s| Cuboid::from_str(s).unwrap()) {
        let mut new_cuboids = HashSet::new();
        for cuboid in cuboid_set {
            for shard in new_cuboid.clone().shatter(&cuboid) {
                new_cuboids.insert(shard);
            }
        }
        new_cuboids.insert(new_cuboid);

        let active_cuboids = new_cuboids
            .iter()
            .filter(|c| c.on)
            .map(|c| c.clone())
            .collect();

        cuboid_set = active_cuboids;
    }
    (
        cuboid_set.iter().map(|c| c.clone().limited_volume()).sum(),
        cuboid_set.iter().map(|c| c.clone().volume()).sum(),
    )
}

pub fn day_22() -> (usize, usize) {
    let input = parse_lines("day_22".to_string());
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
            "on x=-20..26,y=-36..17,z=-47..7".to_string(),
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
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877".to_string(),
            "on x=967..23432,y=45373..81175,z=27513..53682".to_string(),
        ];
        assert_eq!(solve(&input).0, 590784);
    }
}
