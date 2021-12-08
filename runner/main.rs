use days::day_01::day_01;
use days::day_02::day_02;
use days::day_03::day_03;
use days::day_04::day_04;
use days::day_05::day_05;
use days::day_06::day_06;
use days::day_07::day_07;
use days::day_08::day_08;
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

    if args.contains(&"all".to_string()) || args.contains(&"6".to_string()) {
        println!("Running day {}", 6);
        day_06();
    }

    if args.contains(&"all".to_string()) || args.contains(&"7".to_string()) {
        println!("Running day {}", 7);
        day_07();
    }

    if args.contains(&"all".to_string()) || args.contains(&"8".to_string()) {
        println!("Running day {}", 8);
        day_08();
    }
}
