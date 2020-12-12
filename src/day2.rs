use std::collections::HashMap;
use std::io;

fn valid_day2a_password(password: &str, param: char, min_count: usize, max_count: usize) -> bool {
    let mut counts = HashMap::new();

    for c in password.chars() {
        let counter = counts.entry(c).or_insert(0);
        *counter += 1;
    }

    let char_count = match counts.get(&param) {
        Some(n) => *n,
        None => 0,
    };

    char_count >= min_count && char_count <= max_count
}

fn valid_day2b_password(password: &str, param: char, pos_a: usize, pos_b: usize) -> bool {
    (password.chars().nth(pos_a - 1).unwrap() == param)
        ^ (password.chars().nth(pos_b - 1).unwrap() == param)
}

pub fn day2(part_a: bool) {
    // read values from stdin

    let val_fn = if part_a {
        valid_day2a_password
    } else {
        valid_day2b_password
    };

    let mut num_valid = 0;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                let counts: Vec<&str> = parts[0].split('-').collect();
                let min_count = match counts[0].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        panic!("got non-number {}", counts[0]);
                    }
                };
                let max_count = match counts[1].parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        panic!("got non-number {}", counts[0]);
                    }
                };
                let letter = parts[1].chars().next().unwrap();
                let password = parts[2];

                if val_fn(password, letter, min_count, max_count) {
                    num_valid += 1;
                }
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    println!("Number of valid passwords: {}", num_valid);
}
