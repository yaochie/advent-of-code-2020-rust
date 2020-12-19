use std::io;

fn day9b(numbers: &Vec<i64>, prefix_sums: &Vec<i64>, bad_number: i64) {
    // brute force, going through all O(n^2) ranges.
    for (upper_idx, upper_sum) in prefix_sums.iter().enumerate() {
        for (lower_idx, lower_sum) in prefix_sums[..upper_idx].iter().enumerate() {
            if upper_sum - lower_sum == bad_number {
                let range = &numbers[lower_idx..upper_idx];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                let ans = max + min;
                println!("min: {}, max: {}", min, max);
                println!("answer: {}", ans);
                return;
            }
        }
    }
}

pub fn day9() {
    // part 1: find the bad number (using brute force)

    let mut numbers = Vec::new();

    let mut prefix_sums = Vec::new();
    prefix_sums.push(0);
    let mut current_sum = 0;

    let preamble_size = 25;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let next_num = match line.trim().parse::<i64>() {
                    Ok(n) => n,
                    Err(_) => panic!("couldn't parse {}", line)
                };

                numbers.push(next_num);
                current_sum += next_num;
                prefix_sums.push(current_sum);

                if numbers.len() <= preamble_size {
                    continue;
                }

                // check for pair that sums
                let mut has_sum = false;
                for n in &numbers {
                    for m in &numbers {
                        if n != m && n + m == next_num {
                            has_sum = true;
                        }
                    }
                }

                if !has_sum {
                    println!("bad number: {}", next_num);
                    day9b(&numbers, &prefix_sums, next_num);
                    return;
                }
            }
        }
    }
}