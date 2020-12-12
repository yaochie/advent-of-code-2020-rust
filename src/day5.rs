use std::collections::HashSet;
use std::io;

fn get_seat_id(seat: &str) -> u64 {
    if seat.len() != 10 {
        panic!("got seat {} with length {}", seat, seat.len());
    }

    let (row, col) = seat.split_at(7);

    // get row
    let mut row_min: u64 = 0;
    let mut row_max: u64 = 128;

    for c in row.chars() {
        let mid = (row_max + row_min) / 2;

        match c {
            'F' => {
                row_max = mid;
            }
            'B' => {
                row_min = mid;
            }
            _ => panic!("invalid char {}", c),
        }
    }

    // get col
    let mut col_min: u64 = 0;
    let mut col_max: u64 = 8;

    for c in col.chars() {
        let mid = (col_max + col_min) / 2;

        match c {
            'L' => {
                col_max = mid;
            }
            'R' => {
                col_min = mid;
            }
            _ => panic!("invalid char {}", c),
        }
    }

    assert_eq!(row_min, row_max - 1);
    assert_eq!(col_min, col_max - 1);

    row_min * 8 + col_min
}

pub fn day5(part_a: bool) {
    // let mut seats: Vec<u64> = Vec::new();
    let mut seats = HashSet::new();

    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let seat_id = get_seat_id(line.trim());
                seats.insert(seat_id);
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    let min_seat = *seats.iter().min().unwrap();
    let max_seat = *seats.iter().max().unwrap();

    if part_a {
        println!("max seat id: {}", max_seat)
    } else {
        for n in min_seat..=max_seat {
            if !seats.contains(&n) {
                println!("your seat id: {}", n);
                return;
            }
        }
    }
}
