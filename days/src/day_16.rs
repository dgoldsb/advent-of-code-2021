use aoc::from_bin;
use aoc::read_file;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Packet {
    version: usize,
    type_: usize,
    sub_packets: Vec<Packet>,
    literal: usize,
    remainder: String,
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(input: &str) -> Result<Packet, Self::Err> {
        let mut split = input.split_at(3);
        let version = from_bin(&split.0.chars().collect::<Vec<char>>());
        split = split.1.split_at(3);
        let type_ = from_bin(&split.0.chars().collect::<Vec<char>>());
        let mut sub_packets = Vec::new();
        let mut literal = 0;

        let mut remainder = String::new();
        let ref_remainder;
        if type_ == 4 {
            // Literal.
            let mut continue_ = true;
            let mut chars = Vec::new();
            while continue_ {
                split = split.1.split_at(5);
                let bits = split.0.split_at(1);
                continue_ = bits.0 == "1";
                chars.extend(bits.1.chars());
            }
            literal = from_bin(&chars);
            remainder = split.1.to_string();
        } else {
            // Operator.
            split = split.1.split_at(1);
            let length_type_id = split.0 == "0";

            if length_type_id {
                split = split.1.split_at(15);
                let package_size_bits = from_bin(&split.0.chars().collect::<Vec<char>>());
                split = split.1.split_at(package_size_bits);
                ref_remainder = split.1;

                let mut done = false;
                let mut leftover = split.0.to_string();
                while !done {
                    let result = Packet::from_str(&leftover).unwrap();
                    sub_packets.push(result.clone());
                    leftover = result.remainder;
                    done = leftover.len() == 0;
                }
                remainder = ref_remainder.to_string();
            } else {
                split = split.1.split_at(11);
                let package_size_count = from_bin(&split.0.chars().collect::<Vec<char>>());
                let mut leftover = split.1.to_string();

                for _ in 0..package_size_count {
                    let result = Packet::from_str(&leftover).unwrap();
                    sub_packets.push(result.clone());
                    leftover = result.remainder.clone();
                    remainder = result.clone().remainder.clone();
                }
            }
        }

        return Ok(Packet {
            version,
            type_,
            sub_packets,
            literal,
            remainder,
        });
    }
}

fn recursive_sum(input: &Packet) -> usize {
    let mut sum = 0;
    for subpacket in &input.sub_packets {
        sum += recursive_sum(subpacket);
    }
    sum += input.version;
    sum
}

fn recursive_eval(input: &Packet) -> usize {
    let subvalues = input
        .sub_packets
        .iter()
        .map(|p| recursive_eval(p))
        .collect::<Vec<usize>>();

    match input.type_ {
        0 => subvalues.iter().sum(),
        1 => subvalues.iter().fold(1, |a, b| a * b),
        2 => *subvalues.iter().min().unwrap(),
        3 => *subvalues.iter().max().unwrap(),
        4 => input.literal,
        5 => {
            if subvalues.get(0).unwrap() > subvalues.get(1).unwrap() {
                1
            } else {
                0
            }
        }
        6 => {
            if subvalues.get(0).unwrap() < subvalues.get(1).unwrap() {
                1
            } else {
                0
            }
        }
        7 => {
            if subvalues.get(0).unwrap() == subvalues.get(1).unwrap() {
                1
            } else {
                0
            }
        }
        _ => {
            panic!("cannot eval")
        }
    }
}

fn solve(input: &Packet, part_a: bool) -> usize {
    if part_a {
        recursive_sum(input)
    } else {
        recursive_eval(input)
    }
}

pub fn day_16() -> (usize, usize) {
    let mut hb_map = HashMap::new();
    hb_map.insert('0', "0000".to_string());
    hb_map.insert('1', "0001".to_string());
    hb_map.insert('2', "0010".to_string());
    hb_map.insert('3', "0011".to_string());
    hb_map.insert('4', "0100".to_string());
    hb_map.insert('5', "0101".to_string());
    hb_map.insert('6', "0110".to_string());
    hb_map.insert('7', "0111".to_string());
    hb_map.insert('8', "1000".to_string());
    hb_map.insert('9', "1001".to_string());
    hb_map.insert('A', "1010".to_string());
    hb_map.insert('B', "1011".to_string());
    hb_map.insert('C', "1100".to_string());
    hb_map.insert('D', "1101".to_string());
    hb_map.insert('E', "1110".to_string());
    hb_map.insert('F', "1111".to_string());

    let hex_input = read_file("day_16".to_string());
    let bin_input = hex_input
        .chars()
        .map(|c| hb_map.get(&c).unwrap())
        .fold(String::new(), |s, a| s + a);
    let packet = Packet::from_str(&bin_input).unwrap();
    (solve(&packet, true), solve(&packet, false))
}
