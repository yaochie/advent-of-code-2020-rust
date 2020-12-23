use std::collections::HashSet;
use std::io;

// the duplicates probably could be removed using generics?
// or just treating them as vectors..

// type Coordinate = (i64, i64, i64);
type Coordinate = (i64, i64, i64, i64);
// only stores active cubes
type State = HashSet<Coordinate>;

// const OFFSETS: [Coordinate; 26] = [
//     (-1, -1, -1),
//     (-1, -1, 0),
//     (-1, -1, 1),
//     (-1, 0, -1),
//     (-1, 0, 0),
//     (-1, 0, 1),
//     (-1, 1, -1),
//     (-1, 1, 0),
//     (-1, 1, 1),
//     (0, -1, -1),
//     (0, -1, 0),
//     (0, -1, 1),
//     (0, 0, -1),
//     (0, 0, 1),
//     (0, 1, -1),
//     (0, 1, 0),
//     (0, 1, 1),
//     (1, -1, -1),
//     (1, -1, 0),
//     (1, -1, 1),
//     (1, 0, -1),
//     (1, 0, 0),
//     (1, 0, 1),
//     (1, 1, -1),
//     (1, 1, 0),
//     (1, 1, 1),
// ];

const OFFSETS: [Coordinate; 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

// fn add_coordinates((a, b, c): &Coordinate, (d, e, f): &Coordinate) -> Coordinate {
//     (*a + *d, *b + *e, *c + *f)
// }

fn add_coordinates((a, b, c, d): &Coordinate, (e, f, g, h): &Coordinate) -> Coordinate {
    (*a + *e, *b + *f, *c + *g, *d + *h)
}

fn read_state() -> State {
    let mut state = HashSet::new();

    let z = 0i64;
    let w = 0i64;
    let mut y = 0i64;
    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                for (x, c) in line.trim().chars().enumerate() {
                    let coord = (x as i64, y, z, w);

                    match c {
                        '#' => {
                            state.insert(coord);
                        }
                        '.' => (),
                        _ => panic!("found invalid character {}", c),
                    };
                }
            }
        }

        y += 1;
    }

    state
}

/// Counts the number of active cubes in the given state.
fn get_num_active(state: &State) -> u64 {
    state.len() as u64
}

/// Finds the new status of the given cube.
fn new_cube_status(coord: &Coordinate, state: &State) -> bool {
    let mut n_active_neighbours = 0;

    for offset in OFFSETS.iter() {
        let neighbour_coord = add_coordinates(&coord, offset);

        if state.contains(&neighbour_coord) {
            n_active_neighbours += 1;
        }
    }

    let curr_active = state.contains(&coord);

    match (curr_active, n_active_neighbours) {
        (true, 2) | (true, 3) | (false, 3) => true,
        _ => false,
    }
}

/// steps once and returns the new state.
fn step_state(state: State) -> State {
    let mut new_state = HashSet::new();

    for coord in &state {
        for offset in OFFSETS.iter() {
            let new_coord = add_coordinates(coord, offset);

            if new_state.contains(&new_coord) {
                continue;
            }

            if new_cube_status(&new_coord, &state) {
                new_state.insert(new_coord);
            }
        }
    }

    new_state
}

fn day17a(initial_state: State) {
    let n_cycles = 6;

    let mut state = initial_state;
    for cycle_num in 1..(n_cycles + 1) {
        state = step_state(state);

        println!(
            "after {} cycles, number of active states: {}",
            cycle_num,
            get_num_active(&state)
        );
        println!("state size: {}", state.len());
    }
}

pub fn day17() {
    let state = read_state();

    println!("number of active states: {}", get_num_active(&state));

    day17a(state);
}
