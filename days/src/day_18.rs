use aoc::parse_lines;
use aoc::ints_from_str;
use std::default::Default;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

fn is_balanced(s: String) -> bool {
    let mut counter = 0;
    for c in s.chars() {
        match c {
            '[' => {
                counter += 1;
            }
            ']' => {
                counter -= 1;
            }
            _ => {}
        }
    }
    counter == 0
}

#[derive(Clone, Debug)]
struct SnailNumber {
    nest: Option<(Box<SnailNumber>, Box<SnailNumber>)>,
    literal: Option<usize>,
}

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.literal {
            Some(n) => write!(f, "{}", n),
            None => match &self.nest {
                Some(t) => {
                    write!(f, "[{},{}]", t.0, t.1)
                }
                None => write!(f, "[,]"),
            },
        }
    }
}

impl Default for SnailNumber {
    fn default() -> Self {
        SnailNumber {
            nest: Option::None,
            literal: Option::None,
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self == SnailNumber::default() {
            other
        } else {
            let mut number = Self {
                nest: Option::Some((Box::new(self), Box::new(other))),
                ..Default::default()
            };
            number.reduce();
            number
        }
    }
}

impl Sum for SnailNumber {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = SnailNumber>,
    {
        iter.fold(SnailNumber::default(), |a, b| a + b)
    }
}

impl FromStr for SnailNumber {
    type Err = ();

    fn from_str(input: &str) -> Result<SnailNumber, Self::Err> {
        match input.parse::<usize>() {
            Ok(n) => Ok(SnailNumber {
                literal: Option::Some(n),
                ..Default::default()
            }),
            Err(_) => {
                let s = &input[1..input.len() - 1];
                for i in 0..s.len() {
                    let m = &s[i..i+1].chars().next().unwrap();
                    if *m == ',' {
                        let s1 = &s[0..i];
                        let s2 = &s[i + 1..s.len()];

                        if is_balanced(s1.to_string()) && s1.len() > 0 && s2.len() > 0 {
                            return Ok(SnailNumber {
                                nest: Option::Some((
                                    Box::new(SnailNumber::from_str(s1).unwrap()),
                                    Box::new(SnailNumber::from_str(s2).unwrap()),
                                )),
                                ..Default::default()
                            });
                        }
                    }
                }
                panic!(format!("invalid snail number '{}'", input))
            }
        }
    }
}



impl PartialEq for SnailNumber {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl Eq for SnailNumber {}

impl SnailNumber {
    fn reduce(&mut self) {
        let mut current = SnailNumber::default();

        while current != *self {
            current = (*self).clone();
            self.explode();

            if current == *self {
                // Only split if we did not explode.
                self.split();
            }
        }
    }

    fn explode(&mut self) {
        // Explode the first snail number that is recursed too deep.
    }

    fn split(&mut self) {
        // Split the first number larger than 10.
        let string = format!("{}", self);
        let ints = ints_from_str(&string);
        for int in ints {
            if int > 9 {
                let left = int / 2;
                let right = int - left;
                let replacement = format!("[{},{}]", left, right);
                let replaced_string = string.replacen(&format!("{}", int), &replacement, 1);
                let new_snail_number = SnailNumber::from_str(&replaced_string).unwrap();
                self.literal = new_snail_number.literal;
                self.nest = new_snail_number.nest;
                return;
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self.literal {
            Some(n) => n,
            None => {
                let t = self.nest.as_ref().unwrap();
                t.0.magnitude() * 3 + t.1.magnitude() * 2
            }
        }
    }
}

fn solve(input: &Vec<SnailNumber>, part_a: bool) -> usize {
    if part_a {
        println!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        input
            .iter()
            .map(|s| s.clone())
            .sum::<SnailNumber>()
            .magnitude()
    } else {
        0
    }
}

pub fn day_18() -> (usize, usize) {
    let input = parse_lines("day_18".to_string());
    let numbers = input
        .iter()
        .map(|s| SnailNumber::from_str(s).unwrap())
        .collect::<Vec<SnailNumber>>();

    (solve(&numbers, true), solve(&numbers, false))
}

#[cfg(test)]
mod tests {
    use crate::day_18::{is_balanced, solve};
    use crate::day_18::SnailNumber;
    use std::str::FromStr;

    #[test]
    fn simplest_case() {
        let input = vec![SnailNumber::from_str("[1,1]").unwrap()];
        assert_eq!(solve(&input, true), 5);
    }

    #[test]
    fn test_is_balanced() {
        assert_eq!(is_balanced("[[5,5],13]".to_string()), true)
    }

    #[test]
    fn simple_addition() {
        let input = vec![
            SnailNumber::from_str("[1,1]").unwrap(),
            SnailNumber::from_str("[2,2]").unwrap(),
            SnailNumber::from_str("[3,3]").unwrap(),
            SnailNumber::from_str("[4,4]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn split_addition() {
        let input = vec![
            SnailNumber::from_str("[10,13]").unwrap(),
            SnailNumber::from_str("[1,1]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[[5,5],[6,7]],[1,1]]");
    }

    #[test]
    fn explode_addition() {
        let input = vec![
            SnailNumber::from_str("[1,1]").unwrap(),
            SnailNumber::from_str("[2,2]").unwrap(),
            SnailNumber::from_str("[3,3]").unwrap(),
            SnailNumber::from_str("[4,4]").unwrap(),
            SnailNumber::from_str("[5,5]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }
}
