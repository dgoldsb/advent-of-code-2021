use aoc::parse_lines;


fn solve(_input: &Vec<String>, _part_a: bool) -> usize {
    0
}

pub fn day_20() -> (usize, usize) {
    let input = parse_lines("day_20".to_string());
    (solve(&input, true), solve(&input, false))
}
