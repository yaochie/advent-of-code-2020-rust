use std::io;
use std::collections::HashMap;

fn read_day10() -> Vec<i32> {
    // read joltages and sort them
    let mut joltages = Vec::new();
    joltages.push(0);

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let joltage = match line.trim().parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => panic!("couldn't parse {}", line)
                };

                joltages.push(joltage);
            }
        }
    }

    joltages.sort();

    joltages
}

pub fn day10(part_a: bool) {
    let joltages = read_day10();

    if part_a {
        let mut one_diffs = 0;
        let mut three_diffs = 0;

        for (idx, joltage) in joltages[1..].iter().enumerate() {
            let diff = joltage - joltages[idx];
            if diff == 1 {
                one_diffs += 1;
            } else if diff == 3 {
                three_diffs += 1;
            }
        }

        // for the device
        three_diffs += 1;

        println!("1s: {}, 3s: {}, ans: {}", one_diffs, three_diffs, one_diffs * three_diffs);

    } else {
        // dynamic programming
        // let V[i] be the number of paths that include (i.e. end with) joltage i.
        // then V[i] = V[i-1] + V[i-2] + V[i-3].

        let mut num_paths: HashMap<i32, u64> = HashMap::new();
        num_paths.insert(0, 1);

        for joltage in joltages[1..].iter() {
            let mut sum = 0;

            for diff in 1..4 {
                sum += match num_paths.get(&(joltage - diff)) {
                    None => 0,
                    Some(n) => *n
                };
            }

            num_paths.insert(*joltage, sum);
        }

        println!("Answer: {}", num_paths[joltages.last().unwrap()]);
    }
}