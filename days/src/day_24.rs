use aoc::read_file;
use std::str::FromStr;

#[derive(Debug)]
struct SubProgram {
    a: isize,
    b: isize,
    truncate: bool,
}

impl SubProgram {
    // Manually "decompiled".
    fn run(&self, input: &isize, z: &isize) -> isize {
        let mut output: isize = *z;

        // Reduce z.
        if self.truncate {
            output /= 26;
        }

        // Increase z.
        if ((z % 26) + self.a) != *input {
            output *= 26;
            output += input + self.b;
        }

        output
    }

    fn highest_option(&self, z: &isize) -> Option<isize> {
        (1..=9).filter(|i| ((z % 26) + self.a) == *i).max()
    }

    fn lowest_option(&self, z: &isize) -> Option<isize> {
        (1..=9).filter(|i| ((z % 26) + self.a) == *i).min()
    }
}

impl FromStr for SubProgram {
    type Err = ();

    fn from_str(input: &str) -> Result<SubProgram, Self::Err> {
        let lines: Vec<&str> = input.split("\n").collect();
        if lines.len() < 16 {
            return Err(());
        }

        let a: isize = lines
            .get(5)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .parse()
            .unwrap();
        let b: isize = lines
            .get(15)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .parse()
            .unwrap();
        let truncate: bool = lines
            .get(4)
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .parse::<isize>()
            .unwrap()
            == 26;

        return Ok(SubProgram { a, b, truncate });
    }
}

fn run_program(program: &Vec<SubProgram>, inputs: &Vec<isize>) -> isize {
    let mut output = 0;
    for i in 0..14 {
        output = program.get(i).unwrap().run(inputs.get(i).unwrap(), &output);
    }
    output
}

fn solve_recursive(
    maximize: bool,
    program: &Vec<SubProgram>,
    inputs: &mut Vec<isize>,
    z: isize,
) -> Result<Vec<isize>, &'static str> {
    if (inputs.len() == 14) && run_program(program, inputs) == 0 {
        return Ok(inputs.clone());
    } else if inputs.len() == 14 {
        return Err("End result did not match");
    }

    // If truncate is true and we cannot reduce, error out.
    let current_index: usize = inputs.len();
    let subprogram = program.get(current_index).unwrap();
    if subprogram.truncate {
        let best_choice_option = if maximize {
            subprogram.highest_option(&z)
        } else {
            subprogram.lowest_option(&z)
        };
        match best_choice_option {
            Some(best_choice) => {
                inputs.push(best_choice);
                match solve_recursive(maximize, program, inputs, subprogram.run(&best_choice, &z)) {
                    Ok(result) => return Ok(result),
                    Err(_) => inputs.pop(),
                };
            }
            _ => return Err("No match for truncate step"),
        };
    }

    // Very simple strategy, just go over range 9..=1 and hope the search space is
    // small enough with only 7 subprograms to brute force.
    let is = if maximize {
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    for i in is {
        // Push on vector.
        inputs.push(i);

        let result = solve_recursive(maximize, program, inputs, subprogram.run(&i, &z));

        match result {
            Ok(result) => return Ok(result),
            Err(_) => inputs.pop(),
        };
    }
    Err("Ran out of options...")
}

fn solve(program: &Vec<SubProgram>, maximize: bool) -> usize {
    let mut inputs = Vec::new();
    let result = solve_recursive(maximize, program, &mut inputs, 0);
    result
        .unwrap()
        .iter()
        .map(|i| format!("{}", i))
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

pub fn day_24() -> (usize, usize) {
    let input = read_file("day_24".to_string());

    let subprograms: Vec<SubProgram> = input
        .split("inp w")
        .map(|i| SubProgram::from_str(i))
        .filter(|o| o.is_ok())
        .map(|s| s.unwrap())
        .collect::<Vec<SubProgram>>();
    (solve(&subprograms, true), solve(&subprograms, false))
}
