use aoc::parse_ints;
use std::cmp::max;

#[derive(Clone, Debug)]
struct Target {
    x0: isize,
    x1: isize,
    y0: isize,
    y1: isize,
}

fn hits_target(x: isize, y: isize, dx: isize, dy: isize, target: &Target) -> (bool, isize) {
    let point = (x + dx, y + dy);

    if (target.x0 <= point.0 && point.0 <= target.x1)
        && (target.y0 <= point.1 && point.1 <= target.y1)
    {
        return (true, point.1);
    } else if point.0 > target.x1 || (point.1 < target.y0 && dy < 0) {
        return (false, point.1);
    } else {
        let result = hits_target(
            point.0,
            point.1,
            if dx != 0 { dx - (dx / dx.abs()) } else { 0 },
            dy - 1,
            target,
        );
        return (result.0, max(point.1, result.1));
    }
}

fn solve(target: &Target) -> (usize, usize) {
    let mut results = Vec::new();
    for dx in 0..=target.x1 {
        for dy in target.y0..1000 {
            let result = hits_target(0, 0, dx, dy, target);
            if result.0 {
                results.push(result.1);
            }
        }
    }
    (*results.iter().max().unwrap() as usize, results.len())
}

pub fn day_17() -> (usize, usize) {
    let input = parse_ints("day_17".to_string());
    let mut iter = input.iter();
    let target = Target {
        x0: *iter.next().unwrap(),
        x1: *iter.next().unwrap(),
        y0: *iter.next().unwrap(),
        y1: *iter.next().unwrap(),
    };
    solve(&target)
}
