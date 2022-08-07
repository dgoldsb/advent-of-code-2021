use aoc::parse_items;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(input: &str) -> Result<Coordinate, Self::Err> {
        let mut split = input.split(",");
        let coordinate = Coordinate {
            x: split.next().unwrap_or("a").parse().unwrap_or(isize::MAX),
            y: split.next().unwrap_or("a").parse().unwrap_or(isize::MAX),
            z: split.next().unwrap_or("0").parse().unwrap_or(isize::MAX),
        };
        if coordinate.x == isize::MAX || coordinate.y == isize::MAX || coordinate.z == isize::MAX {
            Err(())
        } else {
            Ok(coordinate)
        }
    }
}

#[derive(Clone, Debug)]
struct Sensor {
    location: Coordinate,
    rotation: u8, // 3D space has 24 rotations
    results: HashSet<Coordinate>,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(input: &str) -> Result<Sensor, Self::Err> {
        let location = Coordinate { x: 0, y: 0, z: 0 };
        let rotation = 0;
        let lines = input.split("\n");
        let mut results: HashSet<Coordinate> = HashSet::new();

        for line in lines {
            let result = Coordinate::from_str(line);
            if result.is_ok() {
                results.insert(result.unwrap());
            }
        }

        return Ok(Sensor {
            location,
            rotation,
            results,
        });
    }
}

impl Sensor {
    fn rotate(&self, c: &mut Coordinate) {
        // Z is the face, so we only rotate X and Y.
        let x = c.x;
        let y = c.y;
        let z = c.z;
        match self.rotation {
            // Centre.
            0 => {
                c.x = x;
                c.y = y;
                c.z = z;
            }
            1 => {
                c.x = -y;
                c.y = x;
                c.z = z;
            }
            2 => {
                c.x = y;
                c.y = -x;
                c.z = z;
            }
            3 => {
                c.x = -x;
                c.y = -y;
                c.z = z;
            }
            // Left.
            4 => {
                c.x = x;
                c.z = -y;
                c.y = z;
            }
            5 => {
                c.x = -y;
                c.z = -x;
                c.y = z;
            }
            6 => {
                c.x = y;
                c.z = x;
                c.y = z;
            }
            7 => {
                c.x = -x;
                c.z = y;
                c.y = z;
            }
            // Right.
            8 => {
                c.x = x;
                c.z = y;
                c.y = -z;
            }
            9 => {
                c.x = -y;
                c.z = x;
                c.y = -z;
            }
            10 => {
                c.x = y;
                c.z = -x;
                c.y = -z;
            }
            11 => {
                c.x = -x;
                c.z = -y;
                c.y = -z;
            }
            // 180.
            12 => {
                c.x = x;
                c.y = -y;
                c.z = -z;
            }
            13 => {
                c.x = -y;
                c.y = -x;
                c.z = -z;
            }
            14 => {
                c.x = y;
                c.y = x;
                c.z = -z;
            }
            15 => {
                c.x = -x;
                c.y = y;
                c.z = -z;
            }
            // Up.
            16 => {
                c.z = x;
                c.y = y;
                c.x = -z;
            }
            17 => {
                c.z = -y;
                c.y = x;
                c.x = -z;
            }
            18 => {
                c.z = y;
                c.y = -x;
                c.x = -z;
            }
            19 => {
                c.z = -x;
                c.y = -y;
                c.x = -z;
            }
            // Down.
            20 => {
                c.z = -x;
                c.y = y;
                c.x = z;
            }
            21 => {
                c.z = y;
                c.y = x;
                c.x = z;
            }
            22 => {
                c.z = -y;
                c.y = -x;
                c.x = z;
            }
            23 => {
                c.z = x;
                c.y = -y;
                c.x = z;
            }
            _ => {
                panic!("unknown rotation {}", self.rotation);
            }
        };
    }

    fn shift(&self, c: &mut Coordinate) {
        c.x += self.location.x;
        c.y += self.location.y;
        c.z += self.location.z;
    }

    fn feasible_shifts(&self, cs: &HashSet<Coordinate>) -> HashSet<Coordinate> {
        // Find every shift that makes any combination of points overlap by normalizing.
        let mut set = HashSet::new();

        for s_unrotated in &self.results {
            let mut s = s_unrotated.clone();
            self.rotate(&mut s);
            for o in cs {
                set.insert(Coordinate {
                    x: o.x - s.x,
                    y: o.y - s.y,
                    z: o.z - s.z,
                });
            }
        }
        set
    }

    fn yield_set(&self) -> HashSet<Coordinate> {
        let mut set = HashSet::new();
        for c in &self.results {
            let mut new_c = c.clone();

            self.rotate(&mut new_c);
            self.shift(&mut new_c);
            set.insert(new_c);
        }
        set
    }
}

fn solve(input: &Vec<Sensor>) -> (usize, usize) {
    let mut queue: VecDeque<Sensor> = input
        .iter()
        .map(|r| r.clone())
        .collect::<VecDeque<Sensor>>();
    let mut found_set = HashSet::new();

    // Pop the first, this is our starting point.
    let first = queue.pop_front().unwrap();
    found_set.extend(first.yield_set());

    let mut sensors: Vec<Coordinate> = Vec::new();
    sensors.push(first.location);

    // Now try to match the next with a circular buffer, once the buffer is empty we are done.
    while !queue.is_empty() {
        let mut next = queue.pop_front().unwrap();

        // Check if for any configuration we have `n` overlap.
        let mut found = false;
        for rotation in 0..24 {
            // If we found a match, break.
            if found {
                break;
            }
            // Set the rotation to a new value.
            next.rotation = rotation;

            // TODO: Performance can be found here.
            for shift in next.feasible_shifts(&found_set) {
                // Set the location of the sensor to a new value.
                next.location = shift;

                // Generate the overlap between the sensor's shifted/rotated set and the found set.
                let next_set = next.yield_set();
                let overlap = found_set
                    .intersection(&next_set)
                    .collect::<Vec<&Coordinate>>();

                // If the overlap is large enough we found a match and we exit.
                if overlap.len() >= 12 {
                    found_set.extend(next_set);
                    found = true;
                    sensors.push(next.location.clone());
                    break;
                }
            }
        }

        // If no match was found, add to the back of the circular buffer.
        if !found {
            queue.push_back(next);
        }
    }

    let mut distances: Vec<isize> = Vec::new();
    for sensor_a in &sensors {
        for sensor_b in &sensors {
            distances.push(
                (sensor_a.x - sensor_b.x).abs()
                    + (sensor_a.y - sensor_b.y).abs()
                    + (sensor_a.z - sensor_b.z).abs(),
            );
        }
    }

    let furthest_distance: usize = (*distances.iter().max().unwrap()).try_into().unwrap();

    (found_set.len(), furthest_distance)
}

pub fn day_19() -> (usize, usize) {
    let input = parse_items("day_19".to_string(), "\n\n".to_string());
    let sensors = input
        .iter()
        .map(|s| Sensor::from_str(s).unwrap())
        .collect::<Vec<Sensor>>();
    solve(&sensors)
}
