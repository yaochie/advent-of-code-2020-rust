use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn day15(part_a: bool) {
    // just use brute force?
    let limit = if part_a { 2020 } else { 30000000 };

    let mut line = String::new();
    let mut last_said = HashMap::new();

    match io::stdin().read_line(&mut line) {
        Err(error) => panic!("error: {}", error),
        Ok(0) => {
            println!("got nothing");
            return;
        }
        Ok(_) => (),
    };

    let numbers: Vec<_> = line
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for (idx, n) in numbers.iter().enumerate() {
        last_said.insert(*n, idx + 1);
    }

    // assume the next number is 0, if there are no duplicates in the
    // starting numbers.
    let mut next_num = 0;
    for idx in (numbers.len() + 1)..limit {
        match last_said.get(&next_num) {
            None => {
                last_said.insert(next_num, idx);
                next_num = 0;
            }
            Some(k) => {
                let diff = idx - k;
                last_said.insert(next_num, idx);
                next_num = diff;
            }
        }

        if idx % 500000 == 0 {
            print!("{}...", idx);
            io::stdout().flush().expect("error");
        }
    }

    println!();
    println!("{}th number: {}", limit, next_num);
}
