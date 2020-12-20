use std::collections::HashMap;
use std::io;

use regex::Regex;

fn day14a() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let re = Regex::new(r"^mem\[(.*)\] = (.*)$").unwrap();

    let mut and_mask: u64 = 0;
    let mut set_mask: u64 = 0;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                if line.starts_with("mask") {
                    let mask_str = line[7..].trim();

                    let and_mask_str: String = mask_str
                        .chars()
                        .map(|c| if c == 'X' { '1' } else { '0' })
                        .collect();
                    and_mask = u64::from_str_radix(&and_mask_str, 2).unwrap();

                    let set_mask_str: String = mask_str
                        .chars()
                        .map(|c| if c == 'X' { '0' } else { c })
                        .collect();
                    set_mask = u64::from_str_radix(&set_mask_str, 2).unwrap();
                } else {
                    let caps = re.captures(line.trim()).unwrap();
                    let addr = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

                    memory.insert(addr, (value & and_mask) | set_mask);
                }
            }
        }
    }

    let mut sum = 0;
    for v in memory.values() {
        sum += v;
    }
    println!("sum: {}", sum);
}

fn make_mask(x: u64, positions: &Vec<usize>) -> u64 {
    let mut mask = ['0'; 36];

    let bin_rep: String = format!("{:036b}", x).chars().rev().collect();
    for (idx, c) in bin_rep.chars().enumerate() {
        if idx >= positions.len() {
            break;
        }
        mask[positions[idx]] = c;
    }

    let mask_str: String = mask.iter().rev().collect();
    u64::from_str_radix(&mask_str, 2).unwrap()
}

fn day14b() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let re = Regex::new(r"^mem\[(.*)\] = (.*)$").unwrap();

    let mut or_mask: u64 = 0;
    let mut and_mask: u64 = 0;
    let mut float_positions = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                if line.starts_with("mask") {
                    let mask_str = line[7..].trim();

                    let or_mask_str: String = mask_str
                        .chars()
                        .map(|c| if c == 'X' { '0' } else { c })
                        .collect();
                    or_mask = u64::from_str_radix(&or_mask_str, 2).unwrap();

                    let and_mask_str: String = mask_str
                        .chars()
                        .map(|c| if c == 'X' { '0' } else { '1' })
                        .collect();
                    and_mask = u64::from_str_radix(&and_mask_str, 2).unwrap();

                    float_positions = mask_str
                        .chars()
                        .rev()
                        .enumerate()
                        .filter(|(_, c)| *c == 'X')
                        .map(|(idx, _)| idx)
                        .collect();
                } else {
                    let caps = re.captures(line.trim()).unwrap();
                    let addr = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

                    let float_max = 2u64.pow(float_positions.len() as u32);
                    for i in 0..float_max {
                        let mask = make_mask(i, &float_positions);

                        let new_addr = ((addr & and_mask) | or_mask) | mask;
                        memory.insert(new_addr, value);
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for v in memory.values() {
        sum += v;
    }
    println!("sum: {}", sum);
}

pub fn day14(part_a: bool) {
    if part_a {
        day14a()
    } else {
        day14b()
    }
}
