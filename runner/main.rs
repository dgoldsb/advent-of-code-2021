use days::day_01::day_01;
use days::day_02::day_02;
use days::day_03::day_03;
use days::day_04::day_04;
use days::day_05::day_05;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"all".to_string()) || args.contains(&"1".to_string()) {
        println!("Running day {}", 1);
        day_01();
    }

    if args.contains(&"all".to_string()) || args.contains(&"2".to_string()) {
        println!("Running day {}", 2);
        day_02();
    }

    if args.contains(&"all".to_string()) || args.contains(&"3".to_string()) {
        println!("Running day {}", 3);
        day_03();
    }

    if args.contains(&"all".to_string()) || args.contains(&"4".to_string()) {
        println!("Running day {}", 4);
        day_04();
    }

    if args.contains(&"all".to_string()) || args.contains(&"5".to_string()) {
        println!("Running day {}", 5);
        day_05();
    }
}
