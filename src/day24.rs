use std::collections::HashSet;
use std::io;

fn get_coords(path: String) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let mut chars = path.chars();
    loop {
        match chars.next() {
            None => break,
            Some('e') => x += 1,
            Some('w') => x -= 1,
            Some('s') => {
                y -= 1;
                match chars.next() {
                    Some('e') => x += 1,
                    Some('w') => (),
                    _ => panic!(),
                }
            }
            Some('n') => {
                y += 1;
                match chars.next() {
                    Some('e') => (),
                    Some('w') => x -= 1,
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    (x, y)
}

fn step(black_tiles: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_black_tiles: HashSet<(i32, i32)> = HashSet::new();

    let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1), (1, -1), (-1, 1)];

    // handle black tiles
    for (x, y) in black_tiles {
        let mut n_black_neighbours = 0;
        for (xo, yo) in offsets.iter() {
            if black_tiles.contains(&(x + xo, y + yo)) {
                n_black_neighbours += 1;
            }
        }

        if n_black_neighbours >= 1 && n_black_neighbours <= 2 {
            new_black_tiles.insert((*x, *y));
        }
    }

    // handle white tiles adjacent to black tiles
    for (x, y) in black_tiles {
        for (xo, yo) in offsets.iter() {
            let coord = (x + xo, y + yo);

            let mut n_black_neighbours = 0;
            for (xo2, yo2) in offsets.iter() {
                if black_tiles.contains(&(x + xo + xo2, y + yo + yo2)) {
                    n_black_neighbours += 1;
                }
            }

            if n_black_neighbours == 2 {
                new_black_tiles.insert(coord);
            }
        }
    }

    new_black_tiles
}

pub fn day24() {
    let mut black_tiles = HashSet::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let coords = get_coords(line.trim().to_string());

                if black_tiles.contains(&coords) {
                    black_tiles.remove(&coords);
                } else {
                    black_tiles.insert(coords);
                }
            }
        }
    }

    println!("# of black tiles: {}", black_tiles.len());

    for day in 1..=100 {
        black_tiles = step(&black_tiles);
        println!("Day {}: {}", day, black_tiles.len());
    }
    println!("# black tiles: {}", black_tiles.len());
}
