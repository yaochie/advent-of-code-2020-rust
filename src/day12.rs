use std::io;

enum Angle {
    AntiClockwise,
    Around,
    Clockwise,
}

enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Rotate(Angle),
    Forward(i32),
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn forward(dir: &Direction, distance: &i32, x: &mut i32, y: &mut i32) {
    match dir {
        Direction::North => *y += distance,
        Direction::South => *y -= distance,
        Direction::East => *x += distance,
        Direction::West => *x -= distance,
    }
}

fn turn(dir: &Direction, turn_angle: &Angle) -> Direction {
    match (turn_angle, dir) {
        (Angle::AntiClockwise, Direction::North) => Direction::West,
        (Angle::AntiClockwise, Direction::West) => Direction::South,
        (Angle::AntiClockwise, Direction::South) => Direction::East,
        (Angle::AntiClockwise, Direction::East) => Direction::North,
        (Angle::Clockwise, Direction::North) => Direction::East,
        (Angle::Clockwise, Direction::West) => Direction::North,
        (Angle::Clockwise, Direction::South) => Direction::West,
        (Angle::Clockwise, Direction::East) => Direction::South,
        (Angle::Around, Direction::North) => Direction::South,
        (Angle::Around, Direction::West) => Direction::East,
        (Angle::Around, Direction::South) => Direction::North,
        (Angle::Around, Direction::East) => Direction::West,
    }
}

fn read_actions() -> Vec<Action> {
    let mut actions = Vec::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let arg = match line.trim()[1..].parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => panic!("Failed to parse '{}'", line),
                };
                actions.push(match line.chars().nth(0).unwrap() {
                    'N' => Action::North(arg),
                    'S' => Action::South(arg),
                    'E' => Action::East(arg),
                    'W' => Action::West(arg),
                    'L' => match arg {
                        90 => Action::Rotate(Angle::AntiClockwise),
                        180 => Action::Rotate(Angle::Around),
                        270 => Action::Rotate(Angle::Clockwise),
                        _ => panic!("Want to turn {}", arg),
                    },
                    'R' => match arg {
                        90 => Action::Rotate(Angle::Clockwise),
                        180 => Action::Rotate(Angle::Around),
                        270 => Action::Rotate(Angle::AntiClockwise),
                        _ => panic!("Want to turn {}", arg),
                    },
                    'F' => Action::Forward(arg),
                    _ => panic!(),
                })
            }
        }
    }

    actions
}

fn day12a(actions: &Vec<Action>) {
    let mut x = 0;
    let mut y = 0;

    let mut dir = Direction::East;

    for action in actions {
        match action {
            Action::Forward(n) => forward(&dir, &n, &mut x, &mut y),
            Action::North(n) => y += n,
            Action::South(n) => y -= n,
            Action::East(n) => x += n,
            Action::West(n) => x -= n,
            Action::Rotate(turn_angle) => dir = turn(&mut dir, &turn_angle),
        }
    }

    println!(
        "x: {}, y: {}, Manhattan distance: {}",
        x,
        y,
        x.abs() + y.abs()
    );
}

fn day12b(actions: &Vec<Action>) {
    let mut x = 0;
    let mut y = 0;

    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for action in actions {
        match action {
            Action::North(n) => waypoint_y += n,
            Action::South(n) => waypoint_y -= n,
            Action::East(n) => waypoint_x += n,
            Action::West(n) => waypoint_x -= n,
            Action::Forward(n) => {
                x += waypoint_x * n;
                y += waypoint_y * n;
            }
            Action::Rotate(turn_angle) => match turn_angle {
                Angle::Around => {
                    waypoint_x = -waypoint_x;
                    waypoint_y = -waypoint_y;
                }
                Angle::Clockwise => {
                    let tmp = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -tmp;
                }
                Angle::AntiClockwise => {
                    let tmp = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = tmp;
                }
            },
        }
    }

    println!(
        "x: {}, y: {}, Manhattan distance: {}",
        x,
        y,
        x.abs() + y.abs()
    );
}

pub fn day12(part_a: bool) {
    let actions = read_actions();

    if part_a {
        day12a(&actions);
    } else {
        day12b(&actions);
    }
}
