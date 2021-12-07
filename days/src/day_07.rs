use aoc::parse_ints;

fn absolute_difference(value: isize, data: &Vec<isize>) -> isize {
    data.iter().map(|p| (value - p).abs()).sum()
}

fn crab_difference(value: isize, data: &Vec<isize>) -> isize {
    data.iter()
        .map(|p| (0..=(value - p).abs()).sum::<isize>())
        .sum()
}

fn pick_best(this: isize, other: isize, data: &Vec<isize>, is_a: bool) -> isize {
    if is_a {
        if absolute_difference(this, data) > absolute_difference(other, data) {
            return other;
        } else {
            return this;
        }
    } else {
        if crab_difference(this, data) > crab_difference(other, data) {
            return other;
        } else {
            return this;
        }
    }
}

fn solve(inputs: &Vec<isize>, is_a: bool) -> isize {
    let best_position = (inputs.iter().min().unwrap().clone()
        ..=inputs.iter().max().unwrap().clone())
        .fold(inputs.iter().max().unwrap().clone(), |best, new| {
            pick_best(best, new, inputs, is_a)
        });
    if is_a {
        return absolute_difference(best_position, inputs);
    } else {
        return crab_difference(best_position, inputs);
    }
}

pub fn day_07() {
    let inputs = parse_ints("day_07".to_string());
    println!("A: {}", solve(&inputs, true));
    println!("B: {}", solve(&inputs, false));
}
