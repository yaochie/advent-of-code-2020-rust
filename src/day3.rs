use std::io;

fn read_map() -> Vec<Vec<bool>> {
    let mut map = Vec::new();

    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let row: Vec<bool> = line.trim().chars().map(|c| c == '#').collect();
                map.push(row);
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    map
}

fn count_trees(map: &Vec<Vec<bool>>, row_offset: &usize, col_offset: &usize) -> u128 {
    let height = map.len();
    let width = map[0].len();

    println!("height {}, width {}", height, width);

    let mut r = 0;
    let mut c = 0;

    let mut n_trees = 0;
    while r < height {
        if map[r][c] {
            n_trees += 1;
        }

        // println!("{} {} {}", r, c, n_trees);

        r += row_offset;
        c = (c + col_offset) % width;
    }

    println!("hit {} trees", n_trees);
    n_trees
}

pub fn day3(part_a: bool) {
    let map = read_map();

    if part_a {
        count_trees(&map, &1, &3);
    } else {
        let offsets = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let mut prod: u128 = 1;
        for (col_offset, row_offset) in offsets.iter() {
            let n_trees = count_trees(&map, row_offset, col_offset);
            println!("{} {} {}", col_offset, row_offset, n_trees);
            prod *= n_trees;
        }

        println!("product: {}", prod);
    }
}
