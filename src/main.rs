use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "day1a" => day1::day1(true),
        "day1b" => day1::day1(false),
        "day2a" => day2::day2(true),
        "day2b" => day2::day2(false),
        "day3a" => day3::day3(true),
        "day3b" => day3::day3(false),
        "day4a" => day4::day4(true),
        "day4b" => day4::day4(false),
        "day5a" => day5::day5(true),
        "day5b" => day5::day5(false),
        "day6a" => day6::day6(true),
        "day6b" => day6::day6(false),
        "day7a" => day7::day7(true),
        "day7b" => day7::day7(false),
        "day8a" => day8::day8(true),
        "day8b" => day8::day8(false),
        "day9" => day9::day9(),
        "day10a" => day10::day10(true),
        "day10b" => day10::day10(false),
        "day11a" => day11::day11(true),
        "day11b" => day11::day11(false),
        "day12a" => day12::day12(true),
        "day12b" => day12::day12(false),
        "day13a" => day13::day13(true),
        "day13b" => day13::day13(false),
        "day14a" => day14::day14(true),
        "day14b" => day14::day14(false),
        "day15a" => day15::day15(true),
        "day15b" => day15::day15(false),
        "day16a" => day16::day16(true),
        "day16b" => day16::day16(false),
        "day17" => day17::day17(),
        "day18a" => day18::day18(true),
        "day18b" => day18::day18(false),
        "day19a" => day19::day19(true),
        "day19b" => day19::day19(false),
        _ => {
            panic!("invalid day {}", args[1]);
        }
    }
}
