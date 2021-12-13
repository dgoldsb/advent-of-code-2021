use aoc::binary_grid_from_string;
use aoc::ints_from_str;
use aoc::read_file;
use std::collections::HashSet;

fn pretty_print(set: &HashSet<(isize, isize)>) {
    let max_x = set.iter().map(|t| t.0).max().unwrap();
    let max_y = set.iter().map(|t| t.1).max().unwrap();

    for i in 0..=max_y {
        let mut line = String::new();
        for j in 0..=max_x {
            if set.contains(&(j, i)) {
                line += &"#";
            } else {
                line += &" ";
            }
        }
        println!("{}", line);
    }
}

fn do_fold(set: &mut HashSet<(isize, isize)>, fold: &isize, is_x: &bool) {
    let cloned_set: HashSet<(isize, isize)> = set.clone();
    for t in cloned_set {
        set.remove(&t);
        let mut x = t.0;
        let mut y = t.1;

        if *is_x && x > *fold {
            x = fold - (x - fold);
        } else if !*is_x && y > *fold {
            y = fold - (y - fold);
        }
        set.insert((x, y));
    }
}

fn solve(input: &String, part_a: bool) -> usize {
    let mut i = input.split("\n\n");
    let mut grid = binary_grid_from_string(&i.next().unwrap().to_string());

    let raw_folds = i.next().unwrap().to_string();
    let folds = ints_from_str(&raw_folds);
    let is_x_folds = raw_folds
        .split("\n")
        .map(|l| l.contains("x"))
        .collect::<Vec<bool>>();

    for index in 0..folds.len() {
        let fold = folds.get(index).unwrap();
        let is_x = is_x_folds.get(index).unwrap();

        do_fold(&mut grid, fold, is_x);

        if part_a {
            return grid.len();
        }
    }
    if false {
        pretty_print(&grid);
    }
    0
}

pub fn day_13() -> (usize, usize) {
    let input = read_file("day_13".to_string());
    (solve(&input, true), solve(&input, false))
}
