use aoc::parse_ints;

fn part_a(inputs: &Vec<isize>, window: usize) -> u16 {
    let mut count: u16 = 0;

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

pub fn day_01() {
    let inputs = parse_ints("day_01".to_string());
    println!("A: {}", part_a(&inputs, 1));
    println!("B: {}", part_a(&inputs, 3));
}
