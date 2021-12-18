use aoc::ints_from_str;
use aoc::parse_lines;
use regex::Regex;
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
            let str = format!("[{},{}]", self, other);
            let reduced_number = reduce(&str);
            SnailNumber::from_str(&reduced_number).unwrap()
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
                    let m = &s[i..i + 1].chars().next().unwrap();
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

                // Last resort to resolve syntax error.
                if input.len() > 2 {
                    let mut stripped_input = input.to_string();
                    stripped_input = stripped_input.replace("[[", "[");
                    stripped_input = stripped_input.replace("]]", "]");
                    return SnailNumber::from_str(&stripped_input);
                } else {
                    panic!(format!("invalid snail number '{}'", input));
                }
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

fn reduce(str: &String) -> String {
    let mut current = str.clone();
    let mut previous: String = "".to_string();

    while current != previous {
        println!("{}", current);
        previous = current.clone();
        current = explode(&current);
        if current == previous {
            // Only split if we did not explode.
            current = split(&current);
        }
    }
    current
}

fn explode(str: &String) -> String {
    let re = Regex::new(r"(\[\d+,\d+\])").unwrap();
    let last = Regex::new(r"\D(\d+)\D+$").unwrap();
    let third = Regex::new(r"^\D+\d+\D+\d+\D+(\d+)\D").unwrap();
    let string = str.clone();

    let mut depth = 0;
    for (i, c) in string.chars().enumerate() {
        // Update the depth.
        match c {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
            }
            _ => {
                if depth >= 5 {
                    // Explode at this index!
                    let strings = string.split_at(i - 1);

                    // Find and parse the bit that explodes.
                    let exploding_string = re
                        .captures_iter(strings.1)
                        .next()
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str();
                    let ints = ints_from_str(&exploding_string.to_string());
                    let left = ints.get(0).unwrap();
                    let right = ints.get(1).unwrap();

                    // Find the last integer to the left and add `left` to it, if any.
                    let mut left_str = strings.0.to_string();
                    let found_option = last.captures_iter(&left_str).next();
                    if found_option.is_some() {
                        let full_found = found_option.unwrap().get(0).unwrap().as_str().to_string();
                        let found: isize = *ints_from_str(&full_found).get(0).unwrap();
                        let replacement = found + left;

                        let new_left_str = last.replace(
                            &left_str,
                            &full_found.replace(&found.to_string(), &replacement.to_string()),
                        );
                        left_str = new_left_str.into_owned();
                    }

                    // Find the third integer to the right and add `right` to it, if any.
                    let mut right_str = strings.1.to_string();
                    let found_option = third.captures_iter(&right_str).next();
                    if found_option.is_some() {
                        let full_found = found_option.unwrap().get(0).unwrap().as_str().to_string();
                        let found: isize = *ints_from_str(&full_found).get(2).unwrap();
                        let replacement = found + right;
                        let foo = full_found.replace(&found.to_string(), &replacement.to_string());
                        let replacement_string = re.replace(&foo, &"".to_string());

                        let new_right_str = third.replace(&right_str, &replacement_string);
                        right_str = new_right_str.into_owned();
                    }

                    // Put it all back together and parse.
                    let concat = left_str + "0" + &right_str;

                    return concat;
                }
            }
        }
    }
    return string;
}

fn split(str: &String) -> String {
    // Split the first number larger than 10.
    let string = str.clone();
    let ints = ints_from_str(&string);
    for int in ints {
        if int > 9 {
            let left = int / 2;
            let right = int - left;
            let replacement = format!("[{},{}]", left, right);
            let replaced_string = string.replacen(&format!("{}", int), &replacement, 1);
            return replaced_string;
        }
    }
    return string;
}

impl SnailNumber {
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
    use crate::day_18::SnailNumber;
    use crate::day_18::{is_balanced, solve};
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

    #[test]
    fn explode_right() {
        let input = vec![
            SnailNumber::from_str("[1,1]").unwrap(),
            SnailNumber::from_str("[1,[1,[0,[11,8]]]]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[1,1],[7,[0,6]]]");
    }

    #[test]
    fn harder_test() {
        let input = vec![
            SnailNumber::from_str("[1,1]").unwrap(),
            SnailNumber::from_str("[2,2]").unwrap(),
            SnailNumber::from_str("[3,3]").unwrap(),
            SnailNumber::from_str("[4,4]").unwrap(),
            SnailNumber::from_str("[5,5]").unwrap(),
            SnailNumber::from_str("[6,6]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn hardest_test() {
        panic!("foo");
        let input = vec![
            SnailNumber::from_str("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]").unwrap(),
            SnailNumber::from_str("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]").unwrap(),
            SnailNumber::from_str("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]").unwrap(),
            SnailNumber::from_str("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]").unwrap(),
            SnailNumber::from_str("[7,[5,[[3,8],[1,4]]]]").unwrap(),
            SnailNumber::from_str("[[2,[2,2]],[8,[8,1]]]").unwrap(),
            SnailNumber::from_str("[2,9]").unwrap(),
            SnailNumber::from_str("[1,[[[9,3],9],[[9,0],[0,7]]]]").unwrap(),
            SnailNumber::from_str("[[[5,[7,4]],7],1]").unwrap(),
            SnailNumber::from_str("[[[[4,2],2],6],[8,7]]").unwrap(),
        ];
        let result = format!("{}", input.iter().map(|s| s.clone()).sum::<SnailNumber>());
        assert_eq!(result, "[[[[4,2],2],6],[8,7]]");
    }
}
