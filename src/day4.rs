use std::collections::HashSet;
use std::io;

use regex::Regex;

fn valid_value(key: &str, value: &str) -> bool {
    match key {
        "byr" => match value.parse::<u16>() {
            Ok(n) => n >= 1920 && n <= 2002,
            Err(_) => false,
        },
        "iyr" => match value.parse::<u16>() {
            Ok(n) => n >= 2010 && n <= 2020,
            Err(_) => false,
        },
        "eyr" => match value.parse::<u16>() {
            Ok(n) => n >= 2020 && n <= 2030,
            Err(_) => false,
        },
        "hgt" => {
            if value.ends_with("cm") {
                match value.strip_suffix("cm").unwrap().parse::<u16>() {
                    Ok(height) => height >= 150 && height <= 193,
                    Err(_) => false,
                }
            } else if value.ends_with("in") {
                match value.strip_suffix("in").unwrap().parse::<u16>() {
                    Ok(height) => height >= 59 && height <= 76,
                    Err(_) => false,
                }
            } else {
                return false;
            }
        }
        "hcl" => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value),
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => Regex::new(r"^[0-9]{9}$").unwrap().is_match(value),
        _ => true,
    }
}

fn valid_passport(passport: &str, validate_fields: bool) -> bool {
    let required_fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    let mut fields_present = HashSet::new();
    for part in passport.split_whitespace() {
        let key = part.split(':').nth(0).unwrap();
        let value = part.split(':').nth(1).unwrap();

        if validate_fields && !valid_value(key, value) {
            return false;
        }

        fields_present.insert(key);
    }

    required_fields.is_subset(&fields_present)
}

pub fn day4(part_a: bool) {
    let mut num_valid = 0;

    let mut buf = String::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.trim().len() == 0 {
                    // got newline, so validate passport
                    if valid_passport(&buf, !part_a) {
                        num_valid += 1;
                    }

                    buf = String::new();
                } else {
                    buf.push_str(line.as_str());
                }
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    if buf.len() > 0 && valid_passport(&buf, !part_a) {
        num_valid += 1;
    }

    println!("Number of valid passports: {}", num_valid);
}
