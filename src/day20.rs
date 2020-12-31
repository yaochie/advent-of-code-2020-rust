use std::collections::HashMap;
use std::io;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Clone, Debug)]
struct Tile {
    sides: HashMap<Side, Vec<bool>>,
    tile: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MatchDir {
    Forward,
    Backward,
}

type TileMap = HashMap<u32, Tile>;

fn opposite_side(side: &Side) -> Side {
    match side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
        Side::Top => Side::Bottom,
        Side::Bottom => Side::Top,
    }
}

fn parse_tile(string: &String) -> Tile {
    let mut tile = Vec::new();
    for line in string.trim().split_whitespace() {
        let row: Vec<bool> = line.to_string().chars().map(|c| c == '#').collect();
        tile.push(row);
    }

    let mut sides = HashMap::new();
    sides.insert(Side::Top, tile[0].clone());
    sides.insert(Side::Bottom, tile[9].clone());
    sides.insert(Side::Left, tile.iter().map(|row| row[0]).collect());
    sides.insert(Side::Right, tile.iter().map(|row| row[9]).collect());

    let inner: Vec<Vec<bool>> = tile[1..9].iter().map(|row| row[1..9].to_vec()).collect();

    Tile { sides, tile: inner }
}

fn match_direction(a: &Vec<bool>, b: &Vec<bool>) -> Option<MatchDir> {
    let fwd_match = a.iter().zip(b.iter()).map(|(x, y)| x == y).all(|cmp| cmp);
    let bkd_match = a
        .iter()
        .rev()
        .zip(b.iter())
        .map(|(x, y)| x == y)
        .all(|cmp| cmp);

    if fwd_match {
        Some(MatchDir::Forward)
    } else if bkd_match {
        Some(MatchDir::Backward)
    } else {
        None
    }
}

/// count the number of sides with matches for the given tile
fn count_matches(tile_id: &u32, tile: &Tile, tiles: &TileMap) -> i32 {
    let mut n_match = 0;
    for (_, row) in &tile.sides {
        let mut has_match = false;
        for (tile_id2, tile2) in tiles {
            if tile_id2 == tile_id {
                continue;
            }

            for (_, row2) in &tile2.sides {
                match match_direction(&row, &row2) {
                    None => (),
                    Some(_) => {
                        has_match = true;
                        break;
                    }
                }
            }
            if has_match {
                break;
            }
        }

        if has_match {
            n_match += 1;
        }
    }

    n_match
}

/// find all matches in the tilemap, ignoring tiles with the same id.
fn get_all_matches(
    tile_id: &u32,
    tile: &Tile,
    tiles: &TileMap,
) -> HashMap<Side, (u32, Side, MatchDir)> {
    let mut all_matches: HashMap<Side, (u32, Side, MatchDir)> = HashMap::new();
    for (side, row) in &tile.sides {
        let mut matches = Vec::new();
        for (tile_id2, tile2) in tiles {
            if tile_id2 == tile_id {
                continue;
            }

            for (side2, row2) in &tile2.sides {
                match match_direction(row, row2) {
                    None => (),
                    Some(dir) => {
                        matches.push((tile_id2.clone(), side2.clone(), dir));
                        break;
                    }
                }
            }
        }

        if matches.len() > 1 {
            panic!("Side matches more than 1!! Tile: {}", tile_id);
        }

        if matches.len() == 1 {
            all_matches.insert(side.clone(), matches[0].clone());
        }
    }

    all_matches
}

/// Get part A answer.
fn find_corners(tiles: &TileMap) {
    // count number of sides with matches for each tile.
    // if there are tiles that have only 2 sides with matches,
    // then those must be the corners.
    let mut n_corners = 0;
    let mut n_sides = 0;

    let mut prod: u64 = 1;
    for (tile_id, tile) in tiles {
        let n_matches = count_matches(tile_id, tile, tiles);

        if n_matches == 2 {
            n_corners += 1;
            prod *= (*tile_id) as u64;
            println!("CORNER! tile {}: {} matches", tile_id, n_matches);
        } else if n_matches == 3 {
            n_sides += 1;
            // println!("SIDE!   tile {}: {} matches", tile_id, n_match);
        }
    }
    println!("# corners: {}, # sides: {}", n_corners, n_sides);
    println!("Part A answer: {}", prod);
}

/// Flips the tile on the horizontal axis.
fn flip_horizontal(tile: &Tile) -> Tile {
    let mut new_sides = HashMap::new();
    new_sides.insert(Side::Bottom, tile.sides.get(&Side::Top).unwrap().clone());
    new_sides.insert(Side::Top, tile.sides.get(&Side::Bottom).unwrap().clone());
    new_sides.insert(
        Side::Left,
        tile.sides
            .get(&Side::Left)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );
    new_sides.insert(
        Side::Right,
        tile.sides
            .get(&Side::Right)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );

    let new_tile = flip_image_horizontal(&tile.tile);

    Tile {
        sides: new_sides,
        tile: new_tile,
    }
}

fn flip_image_horizontal(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    image.clone().into_iter().rev().collect()
}

/// Flips the tile on the vertical axis.
fn flip_vertical(tile: &Tile) -> Tile {
    let mut new_sides = HashMap::new();
    new_sides.insert(Side::Left, tile.sides.get(&Side::Right).unwrap().clone());
    new_sides.insert(Side::Right, tile.sides.get(&Side::Left).unwrap().clone());
    new_sides.insert(
        Side::Bottom,
        tile.sides
            .get(&Side::Bottom)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );
    new_sides.insert(
        Side::Top,
        tile.sides
            .get(&Side::Top)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );

    let new_tile = tile
        .tile
        .clone()
        .into_iter()
        .map(|v| v.into_iter().rev().collect())
        .collect();

    Tile {
        sides: new_sides,
        tile: new_tile,
    }
}

/// Rotates tile 90 degrees clockwise.
fn rotate_tile_clockwise(tile: &Tile) -> Tile {
    let mut new_sides = HashMap::new();

    new_sides.insert(
        Side::Top,
        tile.sides
            .get(&Side::Left)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );
    new_sides.insert(Side::Right, tile.sides.get(&Side::Top).unwrap().clone());
    new_sides.insert(
        Side::Bottom,
        tile.sides
            .get(&Side::Right)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );
    new_sides.insert(Side::Left, tile.sides.get(&Side::Bottom).unwrap().clone());

    let new_tile = rotate_image_clockwise(&tile.tile);

    Tile {
        sides: new_sides,
        tile: new_tile,
    }
}

/// Rotates tile 90 degrees anti-clockwise.
fn rotate_anticlockwise(tile: &Tile) -> Tile {
    let mut new_sides = HashMap::new();

    new_sides.insert(Side::Top, tile.sides.get(&Side::Right).unwrap().clone());
    new_sides.insert(
        Side::Right,
        tile.sides
            .get(&Side::Bottom)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );
    new_sides.insert(Side::Bottom, tile.sides.get(&Side::Left).unwrap().clone());
    new_sides.insert(
        Side::Left,
        tile.sides
            .get(&Side::Top)
            .unwrap()
            .clone()
            .into_iter()
            .rev()
            .collect(),
    );

    let new_tile = rotate_image_anticlockwise(&tile.tile);

    Tile {
        sides: new_sides,
        tile: new_tile,
    }
}

fn rotate_image_clockwise(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n_rows = image.len();
    let n_cols = image[0].len();

    let mut new_tile = Vec::new();
    for i in 0..n_rows {
        let mut row = Vec::new();
        for j in (0..n_cols).rev() {
            row.push(image[j][i]);
        }
        new_tile.push(row);
    }

    new_tile
}

fn rotate_image_anticlockwise(image: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n_rows = image.len();
    let n_cols = image[0].len();

    let mut new_tile = Vec::new();
    for i in (0..n_rows).rev() {
        let mut row = Vec::new();
        for j in 0..n_cols {
            row.push(image[j][i]);
        }
        new_tile.push(row);
    }

    new_tile
}

fn get_start_id(tiles: &TileMap) -> Option<u32> {
    let mut start_id = None;
    for (tile_id, tile) in tiles {
        let n_matches = count_matches(tile_id, tile, tiles);
        if n_matches == 2 {
            start_id = Some(tile_id.clone());
        }
    }

    start_id
}

fn add_match(tiles: &TileMap, new_tiles: &mut TileMap, current_id: &u32, match_side: &Side) -> u32 {
    let current_tile = new_tiles.get(current_id).unwrap().clone();

    for (side, (other_id, other_side, match_dir)) in
        get_all_matches(current_id, &current_tile, tiles)
    {
        if side != *match_side {
            continue;
        }

        // flip/rotate the new tile as appropriate
        let next_tile = tiles.get(&other_id).unwrap();

        let fixed_tile = match (&side, &other_side, &match_dir) {
            (Side::Right, Side::Left, MatchDir::Forward)
            | (Side::Bottom, Side::Top, MatchDir::Forward) => next_tile.clone(),

            (Side::Right, Side::Left, MatchDir::Backward)
            | (Side::Bottom, Side::Bottom, MatchDir::Forward) => flip_horizontal(next_tile),

            (Side::Right, Side::Top, MatchDir::Backward)
            | (Side::Bottom, Side::Right, MatchDir::Forward) => rotate_anticlockwise(next_tile),

            (Side::Right, Side::Bottom, MatchDir::Backward)
            | (Side::Bottom, Side::Right, MatchDir::Backward) => {
                flip_horizontal(&rotate_tile_clockwise(next_tile))
            }

            (Side::Right, Side::Right, MatchDir::Forward)
            | (Side::Bottom, Side::Top, MatchDir::Backward) => flip_vertical(next_tile),

            (Side::Right, Side::Right, MatchDir::Backward)
            | (Side::Bottom, Side::Bottom, MatchDir::Backward) => {
                rotate_tile_clockwise(&rotate_tile_clockwise(next_tile))
            }

            (Side::Right, Side::Bottom, MatchDir::Forward)
            | (Side::Bottom, Side::Left, MatchDir::Backward) => rotate_tile_clockwise(next_tile),

            (Side::Right, Side::Top, MatchDir::Forward)
            | (Side::Bottom, Side::Left, MatchDir::Forward) => {
                flip_vertical(&rotate_tile_clockwise(next_tile))
            }

            (Side::Top, _, _) => panic!("{:?}, {:?}, {:?}", side, other_side, match_dir),
            (Side::Left, _, _) => panic!("{:?}, {:?}, {:?}", side, other_side, match_dir),
        };

        assert_eq!(
            match_direction(
                current_tile.sides.get(match_side).unwrap(),
                fixed_tile.sides.get(&opposite_side(match_side)).unwrap()
            ),
            Some(MatchDir::Forward)
        );
        new_tiles.insert(other_id, fixed_tile);

        return other_id;
    }

    panic!()
}

/// compute offsets for the pattern for easier matching later
fn prepare_pattern() -> Vec<usize> {
    let pattern = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
    println!("{}", pattern);
    let lines: Vec<_> = pattern.split('\n').collect();

    println!("# rows: {}", lines.len());
    for line in &lines {
        println!("# cols: {}", line.len());
    }

    let width = 8 * 12;
    let mut offsets = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                offsets.push(i * width + j);
            }
        }
    }

    println!("{:?}", offsets);
    offsets
}

fn build_image(tiles: &TileMap, tile_positions: &HashMap<(u32, u32), u32>) -> Vec<Vec<bool>> {
    let mut image = Vec::new();
    for _ in 0..96 {
        image.push(Vec::new());
    }

    for row in 0..12 {
        for col in 0..12 {
            let tile_id = tile_positions.get(&(row, col)).unwrap();
            let tile = tiles.get(tile_id).unwrap();

            let inner = &tile.tile;
            for (r, inner_row) in inner.iter().enumerate() {
                image[(row as usize) * 8 + r].append(&mut inner_row.clone());
            }
        }
    }

    image
}

fn flatten_vec(image: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut final_image = Vec::new();
    for row in image.iter() {
        let mut row = row.clone();
        final_image.append(&mut row);
    }

    final_image
}

/// fits tiles together, rotating/flipping them as necessary.
/// returns a mapping of coordinates to tile_ids and the rotated tiles.
fn fit_tiles(tiles: &TileMap) -> (HashMap<(u32, u32), u32>, TileMap) {
    // first, find a corner, then flip/rotate so that its sides
    // with matches are at the right and bottom.

    let mut rotated_tiles: TileMap = HashMap::new();

    let start_id = get_start_id(tiles).unwrap();

    let start_tile = tiles.get(&start_id).unwrap();
    let start_matches = get_all_matches(&start_id, start_tile, tiles);

    let fixed_start_tile = match (
        start_matches.contains_key(&Side::Left),
        start_matches.contains_key(&Side::Right),
        start_matches.contains_key(&Side::Top),
        start_matches.contains_key(&Side::Bottom),
    ) {
        (false, true, false, true) => start_tile.clone(),
        (true, false, false, true) => flip_vertical(start_tile),
        (false, true, true, false) => flip_horizontal(start_tile),
        (true, false, true, false) => flip_horizontal(&flip_vertical(start_tile)),
        _ => panic!(),
    };

    rotated_tiles.insert(start_id, fixed_start_tile);
    let mut board = HashMap::new();

    board.insert((0, 0), start_id);

    // keep going right until we reach a corner, then go to the next row
    // repeat until done.

    let mut row_start_id = start_id;
    for row in 0..12 {
        let mut curr_id = row_start_id;
        for col in 1..12 {
            let next_id = add_match(tiles, &mut rotated_tiles, &curr_id, &Side::Right);

            board.insert((row, col), next_id);
            curr_id = next_id;
        }

        if row < 11 {
            let next_id = add_match(tiles, &mut rotated_tiles, &row_start_id, &Side::Bottom);

            row_start_id = next_id;
            board.insert((row + 1, 0), next_id);
        }
    }

    (board, rotated_tiles)
}

fn num_monsters(image: &Vec<bool>, offsets: &Vec<usize>) -> i32 {
    let pattern_height = 3;
    let pattern_width = 20;

    let max_row = 96 - pattern_height;
    let max_col = 96 - pattern_width;

    let mut n_match = 0;
    for row in 0..max_row {
        for col in 0..max_col {
            let idx = row * 96 + col;

            let mut is_match = true;
            for offset in offsets {
                if !image[idx + offset] {
                    is_match = false;
                }
            }

            if is_match {
                println!("found match at ({}, {})", row, col);
                n_match += 1;
            }
        }
    }

    let total_occupied: i32 = image.iter().map(|cell| if *cell { 1 } else { 0 }).sum();

    if n_match > 0 {
        println!("# matches: {}", n_match);
        println!(
            "Answer: {}",
            total_occupied - n_match * (offsets.len() as i32)
        );
    }

    n_match
}

fn calc_part_b(tiles: &TileMap, offsets: &Vec<usize>) {
    // calculate number of matching sides.
    // seems each side only matches at most one other side.
    let mut n_sides = HashMap::new();

    for (tile_id, tile) in tiles {
        let n_matches = count_matches(tile_id, tile, tiles);
        get_all_matches(tile_id, tile, tiles);
        n_sides.insert(tile_id.clone(), n_matches.clone());
    }

    for (tile_id, _) in n_sides.iter().filter(|(_, n_matches)| **n_matches == 2) {
        println!("{}", tile_id);
        let matches = get_all_matches(tile_id, tiles.get(tile_id).unwrap(), tiles);

        println!("matches:");
        for (side, (other_id, other_side, match_dir)) in &matches {
            println!(
                "{:?}: [{} {:?} {:?}]",
                side, other_id, other_side, match_dir
            );
        }
        println!();
    }

    // pick one corner to put at (0, 0),
    // then slowly fill in the sides.
    // then fill in the middle

    let (positions, new_tiles) = fit_tiles(tiles);

    // construct the final image, then rotate and flip
    // until we see monsters.

    let image = build_image(&new_tiles, &positions);
    let mut rot_image = image;

    for _ in 0..4 {
        if num_monsters(&flatten_vec(&rot_image), offsets) > 0 {
            return;
        }
        rot_image = rotate_image_clockwise(&rot_image);
    }

    rot_image = flip_image_horizontal(&rot_image);
    for _ in 0..4 {
        if num_monsters(&flatten_vec(&rot_image), offsets) > 0 {
            return;
        }
        rot_image = rotate_image_clockwise(&rot_image);
    }
}

fn read_input() -> TileMap {
    let mut tiles = HashMap::new();

    let mut n_tiles = 0;
    loop {
        let mut title = String::new();
        match io::stdin().read_line(&mut title) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => (),
        }

        let title_id = title[5..9].parse::<u32>().unwrap();

        let mut tile = String::new();
        for _ in 0..11 {
            io::stdin().read_line(&mut tile).unwrap();
        }

        tiles.insert(title_id, parse_tile(&tile));
        n_tiles += 1;
    }

    println!("# tiles: {}", n_tiles);
    tiles
}

pub fn day20(part_a: bool) {
    let tiles = read_input();

    if part_a {
        find_corners(&tiles);
    } else {
        let pattern_offsets = prepare_pattern();
        calc_part_b(&tiles, &pattern_offsets);
    }
}
