use aoc::parse_ints;

fn part_a(inputs: &Vec<isize>, window: usize) -> usize {
    let mut count: usize = 0;

    for i in 1..(inputs.len() + 1 - window) {
        let mut cum_a = 0;
        for j in (i - 1)..(i - 1 + window) {
            cum_a += inputs[j];
        }

        let mut cum_b = 0;
        for k in i..(i + window) {
            cum_b += inputs[k];
        }

        if cum_b > cum_a {
            count += 1;
        }
    }
    count
}

pub fn day_01() -> (usize, usize) {
    let inputs = parse_ints("day_01".to_string());
    (part_a(&inputs, 1), part_a(&inputs, 3))
}
