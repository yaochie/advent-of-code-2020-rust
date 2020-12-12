use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "day1a" => day1::day1a(),
        "day1b" => day1::day1b(),
        "day2a" => day2::day2(true),
        "day2b" => day2::day2(false),
        _ => {
            panic!("invalid day {}", args[1]);
        }
    }
}
