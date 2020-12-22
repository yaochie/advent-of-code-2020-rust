use std::collections::HashMap;
use std::io;
use std::ops::Range;

use regex::Regex;

#[derive(Clone)]
struct Field {
    range1: Range<i32>,
    range2: Range<i32>,
}

/// checks if a number is within the valid ranges for the field.
fn valid_for_field(field: &Field, x: &i32) -> bool {
    field.range1.contains(x) || field.range2.contains(x)
}

/// returns None if no fields are invalid,
/// else returns the sum of the invalid fields
fn invalid_sum(fields: &HashMap<String, Field>, ticket: &Vec<i32>) -> Option<i32> {
    let mut sum = 0;
    let mut valid_ticket = true;

    for v in ticket {
        let mut valid_value = false;
        for (_, field) in fields {
            if valid_for_field(field, v) {
                valid_value = true;
                break;
            }
        }

        if !valid_value {
            valid_ticket = false;
            sum += v;
        }
    }

    if valid_ticket {
        None
    } else {
        Some(sum)
    }
}

fn read_fields() -> HashMap<String, Field> {
    let mut fields = HashMap::new();

    let field_re = Regex::new(r"^(.*): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                if line.trim().len() == 0 {
                    break;
                }

                let caps = field_re.captures(line.trim()).unwrap();
                let name = caps.get(1).unwrap().as_str().to_string();
                let start1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let end1 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let start2 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                let end2 = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();

                fields.insert(
                    name,
                    Field {
                        range1: start1..(end1 + 1),
                        range2: start2..(end2 + 1),
                    },
                );
            }
        }
    }

    fields
}

fn parse_ticket(ticket: &String) -> Vec<i32> {
    ticket
        .trim()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

fn day16a() {
    let fields = read_fields();

    // our ticket
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();

    let ticket = line.trim();

    println!("{}", ticket);

    // nearby tickets
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();

    let mut sum = 0;
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let ticket: Vec<i32> = parse_ticket(&line);

                sum += match invalid_sum(&fields, &ticket) {
                    None => 0,
                    Some(n) => n,
                };
            }
        }
    }

    println!("{}", sum);
}

/// given all tickets, fields remaining to be assigned,
/// and currently assigned positions,
/// find a new position.
fn get_position(
    tickets: &Vec<Vec<i32>>,
    max_position: usize,
    fields: &HashMap<String, Field>,
    assigned_positions: &HashMap<usize, String>,
) -> (String, usize) {
    let mut possible_positions = HashMap::new();

    for (name, field) in fields {
        for pos in 0..max_position {
            if assigned_positions.contains_key(&pos) {
                continue;
            }

            let mut valid_pos = true;
            for ticket in tickets {
                if !valid_for_field(field, &ticket[pos]) {
                    valid_pos = false;
                    break;
                }
            }

            if valid_pos {
                match possible_positions.get_mut(name) {
                    None => {
                        let new_set: Vec<_> = [pos].iter().cloned().collect();
                        possible_positions.insert(name, new_set);
                    }
                    Some(set) => {
                        set.push(pos);
                    }
                }
            }
        }

        match possible_positions.get(name) {
            None => panic!("couldn't find position for {}", name),
            Some(set) => {
                if set.len() == 1 {
                    println!("found position {} for {}", set[0], name);
                    return (name.clone(), set[0]);
                }
            }
        }
    }

    panic!("no field with only 1 possible position!");
}

fn day16b() {
    let fields = read_fields();

    // our ticket
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.clear();
    io::stdin().read_line(&mut line).unwrap();

    let ticket = parse_ticket(&line);
    let max_positions = ticket.len();

    println!("{}", line.trim());
    println!("ticket has {} positions", max_positions);

    // nearby tickets
    io::stdin().read_line(&mut line).unwrap();
    io::stdin().read_line(&mut line).unwrap();

    let mut discarded_tickets = 0;
    let mut valid_tickets = Vec::new();
    loop {
        line.clear();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let ticket = parse_ticket(&line);

                match invalid_sum(&fields, &ticket) {
                    None => valid_tickets.push(ticket),
                    Some(_) => discarded_tickets += 1,
                };
            }
        }
    }

    println!("discarded {} tickets", discarded_tickets);
    println!("{} valid tickets left", valid_tickets.len());

    // see if this works: find a field that has only one possible position,
    // then set it, and keep looping.

    let mut field_positions = HashMap::new();
    let mut fields_to_assign = fields.clone();
    while field_positions.len() < max_positions {
        let new_position = get_position(
            &valid_tickets,
            max_positions,
            &fields_to_assign,
            &field_positions,
        );
        match fields_to_assign.remove(&new_position.0) {
            None => panic!("position assigned to non-existent field {}", new_position.0),
            Some(_) => (),
        }
        field_positions.insert(new_position.1, new_position.0);
    }

    println!("------");
    let mut prod = 1;
    for (pos, field_name) in field_positions.iter() {
        if field_name.starts_with("departure") {
            println!(
                "{}: {} (our ticket value: {})",
                field_name, pos, ticket[*pos]
            );
            prod *= ticket[*pos] as i128;
        }
    }
    println!("answer: {}", prod);
}

pub fn day16(part_a: bool) {
    if part_a {
        day16a()
    } else {
        day16b()
    }
}
