use days::day_01::day_01;
use days::day_02::day_02;
use days::day_03::day_03;
use days::day_04::day_04;
use days::day_05::day_05;
use days::day_06::day_06;
use days::day_07::day_07;
use days::day_08::day_08;
use days::day_09::day_09;
use days::day_10::day_10;
use days::day_11::day_11;
use days::day_12::day_12;
use days::day_13::day_13;
use days::day_14::day_14;
use days::day_15::day_15;
use days::day_16::day_16;
use days::day_17::day_17;
use days::day_18::day_18;
use days::day_19::day_19;
use days::day_20::day_20;
use days::day_21::day_21;
use days::day_22::day_22;
use days::day_23::day_23;
use days::day_24::day_24;
use days::day_25::day_25;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut methods: HashMap<String, fn() -> (usize, usize)> = HashMap::new();
    methods.insert("01".to_string(), day_01);
    methods.insert("02".to_string(), day_02);
    methods.insert("03".to_string(), day_03);
    methods.insert("04".to_string(), day_04);
    methods.insert("05".to_string(), day_05);
    methods.insert("06".to_string(), day_06);
    methods.insert("07".to_string(), day_07);
    methods.insert("08".to_string(), day_08);
    methods.insert("09".to_string(), day_09);
    methods.insert("10".to_string(), day_10);
    methods.insert("11".to_string(), day_11);
    methods.insert("12".to_string(), day_12);
    methods.insert("13".to_string(), day_13);
    methods.insert("14".to_string(), day_14);
    methods.insert("15".to_string(), day_15);
    methods.insert("16".to_string(), day_16);
    methods.insert("17".to_string(), day_17);
    methods.insert("18".to_string(), day_18);
    methods.insert("19".to_string(), day_19);
    methods.insert("20".to_string(), day_20);
    methods.insert("21".to_string(), day_21);
    methods.insert("22".to_string(), day_22);
    methods.insert("23".to_string(), day_23);
    methods.insert("24".to_string(), day_24);
    methods.insert("25".to_string(), day_25);

    let mut days = args.iter().collect::<Vec<&String>>();
    days.remove(0);
    if days.is_empty() {
        days.extend(methods.keys());
    }
    days.sort();
    let start = Instant::now();
    println!(
        "{0: <4} | {1: <20} | {2: <20} | {3: <20}",
        "Day", "Part A", "Part B", "Runtime"
    );
    for day in days {
        let now = Instant::now();
        let t = match methods.get(day) {
            Some(f) => f(),
            None => panic!("unknown day"),
        };
        let runtime = format!(
            "{}.{} ms",
            now.elapsed().as_millis(),
            now.elapsed().as_nanos() % 1000000
        );
        println!(
            "{0: <4} | {1: <20} | {2: <20} | {3: <20}",
            day, t.0, t.1, runtime,
        );
    }
    println!("\nTotal {} ms", start.elapsed().as_millis());
}
