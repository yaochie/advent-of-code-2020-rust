use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::time::Instant;

fn print_config(cups: &VecDeque<u64>) {
    let s: Vec<_> = cups.iter().map(|c| format!("{}", c)).collect();
    println!("configuration: {}", s.join(","));
}

// fn print_map_config(map: &HashMap<u64, u64>, start: &u64) {
//     print!("configuration: ");

//     let mut curr = start;
//     for _ in 0..map.len() {
//         print!("{},", curr);
//         curr = map.get(curr).unwrap();
//     }
//     println!();
// }

fn make_move(cups: &mut VecDeque<u64>, max_val: u64) {
    let cup = cups[0];

    cups.rotate_left(1);

    let c1 = cups.pop_front().unwrap();
    let c2 = cups.pop_front().unwrap();
    let c3 = cups.pop_front().unwrap();

    // find next
    let mut next = if cup == 1 { max_val } else { cup - 1 };
    while next == c1 || next == c2 || next == c3 {
        next = if next == 1 { max_val } else { next - 1 };
    }

    let mut insert_idx = 0;
    for (i, c) in cups.iter().enumerate() {
        if *c == next {
            insert_idx = i + 1;
            break;
        }
    }

    cups.insert(insert_idx, c3);
    cups.insert(insert_idx, c2);
    cups.insert(insert_idx, c1);
}

fn make_move_map(map: &mut HashMap<u64, u64>, curr: u64, max_val: u64) -> u64 {
    let c1 = map.get(&curr).unwrap().clone();
    let c2 = map.get(&c1).unwrap().clone();
    let c3 = map.get(&c2).unwrap().clone();

    let c4 = map.get(&c3).unwrap().clone();

    // remove
    map.insert(curr, c4);

    // find next
    let mut next = if curr == 1 { max_val } else { curr - 1 };
    while next == c1 || next == c2 || next == c3 {
        next = if next == 1 { max_val } else { next - 1 };
    }

    // insert
    let other = map.get(&next).unwrap().clone();

    map.insert(next, c1);
    map.insert(c3, other);

    c4
}

/// store the cups in a simple VecDeque.
/// this is slower, but good enough for the simple game.
fn simple_game(start: String) {
    let mut cups: VecDeque<u64> = start
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect();

    for _ in 0..100 {
        make_move(&mut cups, 9);
    }
    print!("Final ");
    print_config(&cups);
}

/// store the cups in a singly-linked circular-list hash map.
/// then moving the cups just involves modifying the `next`
/// reference.
///
/// This probably isn't the fastest method: still takes about
/// a minute to run in debug mode (<10s in release mode).
fn complex_game(start: String) {
    println!("{}", start);

    let max_val: usize = 1000000;
    let n_moves = 10000000;

    let mut cups: Vec<u64> = (1..=max_val as u64).collect();
    for (i, c) in start.chars().enumerate() {
        cups[i] = c.to_string().parse::<u64>().unwrap();
    }

    assert_eq!(cups.len(), max_val);

    let mut map = HashMap::new();
    for (i, c) in cups.iter().enumerate() {
        map.insert(c.clone(), cups[(i + 1) % max_val]);
    }

    assert_eq!(map.len(), max_val);

    let mut curr = cups[0];
    for _ in 0..n_moves {
        curr = make_move_map(&mut map, curr, max_val as u64);
        // print_map_config(&map, &curr);
    }

    // find cups after 1.
    let first = map.get(&1).unwrap();
    let second = map.get(&first).unwrap();

    println!("cups after 1: {} {}", first, second);
    println!("answer: {}", first * second);
}

pub fn day23(part_a: bool) {
    let start = env::args().nth(2).unwrap();
    println!("initial configuration: {}", start);

    let now = Instant::now();
    if part_a {
        simple_game(start);
    } else {
        complex_game(start);
    }
    println!("time taken: {}ms", now.elapsed().as_millis());
}
