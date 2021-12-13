use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn ints_from_str(input: &String) -> Vec<isize> {
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
        .collect()
}

pub fn binary_grid_from_string(input: &String) -> HashSet<(isize, isize)> {
    let pairs = input
        .split("\n")
        .map(|l| ints_from_str(&l.to_string()))
        .collect::<Vec<Vec<isize>>>();

    let mut set = HashSet::new();
    pairs.iter().for_each(|v| {
        set.insert((v.get(0).unwrap().clone(), v.get(1).unwrap().clone()));
    });
    set
}

pub fn read_file(day: String) -> String {
    fs::read_to_string(format!("input/{}.txt", day)).expect("Something went wrong reading the file")
}

pub fn parse_chars(day: String) -> Vec<char> {
    read_file(day).chars().collect()
}

pub fn parse_ints(day: String) -> Vec<isize> {
    let re = Regex::new(r"([-+]?\d+)\D?").unwrap();
    let input = read_file(day);
    re.captures_iter(&input)
        .map(|c| c[1].parse().expect("Something went wrong parsing an int"))
        .collect()
}

pub fn parse_items(day: String, delimiter: String) -> Vec<String> {
    read_file(day)
        .split(&delimiter)
        .map(|i| i.to_string())
        .collect()
}

pub fn parse_lines(day: String) -> Vec<String> {
    read_file(day).split("\n").map(|i| i.to_string()).collect()
}

pub fn parse_u32_map(day: String) -> HashMap<(i32, i32), u32> {
    let input = parse_lines(day)
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut map = HashMap::new();
    for (i, l) in input.iter().enumerate() {
        for (j, v) in l.iter().enumerate() {
            map.insert((i as i32, j as i32), v.clone());
        }
    }
    map
}

pub fn to_bin(number: &usize) -> Vec<char> {
    return format!("{:0>36}", format!("{:b}", number))
        .chars()
        .collect();
}

pub fn from_bin(bin_number: &Vec<char>) -> usize {
    let bin_str: String = bin_number.into_iter().collect();
    return usize::from_str_radix(&bin_str, 2).unwrap();
}
