use std::io;

#[derive(Clone, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn read_seats() -> (Vec<Seat>, i32, i32) {
    let mut seats = Vec::new();

    let mut width = 0;
    let mut height = 0;

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                width = line.trim().len() as i32;
                for c in line.trim().chars() {
                    match c {
                        'L' => seats.push(Seat::Empty),
                        '.' => seats.push(Seat::Floor),
                        _ => panic!("got unexpected input `{}`", c),
                    }
                }
            }
        }
        height += 1;
    }

    (seats, width, height)
}

fn count_occupied(seats: &Vec<Seat>) -> i32 {
    seats
        .into_iter()
        .map(|seat| match seat {
            Seat::Occupied => 1,
            _ => 0,
        })
        .sum()
}

fn step_adj(seats: &Vec<Seat>, width: i32, height: i32) -> Vec<usize> {
    // return the indexes to change

    let mut changes = Vec::new();

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (idx, seat) in seats.iter().enumerate() {
        let row = (idx as i32) / width;
        let col = (idx as i32) % width;

        let mut adj_occupied = 0;

        if *seat == Seat::Floor {
            continue;
        }

        for (row_offset, col_offset) in offsets.iter() {
            let new_row = row + row_offset;
            let new_col = col + col_offset;

            if new_row < 0 || new_col < 0 || new_row >= height || new_col >= width {
                continue;
            }

            if seats[(new_row * width + new_col) as usize] == Seat::Occupied {
                adj_occupied += 1;
            }
        }

        if *seat == Seat::Empty && adj_occupied == 0 {
            changes.push(idx);
        } else if *seat == Seat::Occupied && adj_occupied >= 4 {
            changes.push(idx);
        }
    }

    changes
}

fn step_visible(seats: &Vec<Seat>, width: i32, height: i32) -> Vec<usize> {
    // return the indexes to change

    let mut changes = Vec::new();

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (idx, seat) in seats.iter().enumerate() {
        let row = (idx as i32) / width;
        let col = (idx as i32) % width;

        let mut adj_occupied = 0;

        if *seat == Seat::Floor {
            continue;
        }

        for (row_offset, col_offset) in offsets.iter() {
            let mut new_row = row + row_offset;
            let mut new_col = col + col_offset;

            // add offset until we reach a seat or go out of bounds
            while new_row >= 0
                && new_col >= 0
                && new_row < height
                && new_col < width
                && seats[(new_row * width + new_col) as usize] == Seat::Floor
            {
                new_row += row_offset;
                new_col += col_offset;
            }

            if new_row < 0 || new_col < 0 || new_row >= height || new_col >= width {
                continue;
            }

            if seats[(new_row * width + new_col) as usize] == Seat::Occupied {
                adj_occupied += 1;
            }
        }

        if *seat == Seat::Empty && adj_occupied == 0 {
            changes.push(idx);
        } else if *seat == Seat::Occupied && adj_occupied >= 5 {
            changes.push(idx);
        }
    }

    changes
}

pub fn day11(part_a: bool) {
    let (mut seats, width, height) = read_seats();

    let step_fn = if part_a { step_adj } else { step_visible };

    let mut n_rounds = 0;
    loop {
        n_rounds += 1;
        let new_seats = step_fn(&seats, width, height);

        if new_seats.len() == 0 {
            break;
        }

        for i in new_seats {
            match seats[i] {
                Seat::Floor => panic!("want to toggle floor"),
                Seat::Occupied => seats[i] = Seat::Empty,
                Seat::Empty => seats[i] = Seat::Occupied,
            }
        }
        println!("ran for {} rounds", n_rounds);
        println!("Number of occupied seats: {}", count_occupied(&seats));
    }

    println!("ran for {} rounds", n_rounds);
    println!("Number of occupied seats: {}", count_occupied(&seats));
}
