use days::day_01::day_01;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"all".to_string()) || args.contains(&"1".to_string()) {
        println!("Running day {}", 1);
        day_01();
    }
}
