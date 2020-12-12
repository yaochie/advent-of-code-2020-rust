use std::collections::HashSet;
use std::io;

fn read_day1() -> HashSet<i32> {
    // read values from stdin
    let mut values = HashSet::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                // reached end of file.
                break;
            }
            Ok(_) => {
                let value = match line.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        panic!("got non-number in input {}", line);
                    }
                };

                // check if all numbers are unique
                if values.contains(&value) {
                    println!("already have {}!", value);
                }

                values.insert(value);
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
    }

    values
}

fn day1a(total: i32, values: &HashSet<i32>) {
    let total = 2020;
    let values = read_day1();

    for value in &values {
        let other_value = total - value;

        if values.contains(&other_value) {
            println!("found pair {} and {}", value, other_value);
            println!("product: {}", value * other_value);
            return;
        }
    }
}

fn day1b(total: i32, values: &HashSet<i32>) {
    let total = 2020;
    let values = read_day1();

    for a in &values {
        for b in &values {
            if a != b {
                let remainder = total - a - b;

                if values.contains(&remainder) {
                    println!("found triplet {} and {} and {}", a, b, remainder);
                    println!("product: {}", a * b * remainder);
                    return;
                }
            }
        }
    }
}

pub fn day1(part_a: bool) {
    let total = 2020;
    let values = read_day1();

    if part_a {
        day1a(total, &values)
    } else {
        day1b(total, &values)
    }
}
