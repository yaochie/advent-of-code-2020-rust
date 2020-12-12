use std::collections::HashSet;
use std::io;

fn count_group_any(group: &str) -> usize {
    let persons: Vec<HashSet<char>> = group
        .split_whitespace()
        .map(|person| person.chars().collect())
        .collect();

    // union all
    let group_answers = persons.iter().fold(HashSet::new(), |acc, person| {
        acc.union(person).cloned().collect()
    });

    group_answers.len()
}

fn count_group_all(group: &str) -> usize {
    let persons: Vec<HashSet<char>> = group
        .split_whitespace()
        .map(|person| person.chars().collect())
        .collect();

    // intersect all
    let first = persons[0].clone();

    let group_answers = persons.iter().skip(1).fold(first, |acc, person| {
        acc.intersection(person).cloned().collect()
    });

    group_answers.len()
}

pub fn day6(part_a: bool) {
    let mut total: usize = 0;
    let mut buf = String::new();

    let count_fn = if part_a {
        count_group_any
    } else {
        count_group_all
    };

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if line.trim().len() == 0 {
                    // got newline, so count for the group
                    total += count_fn(&buf);

                    buf = String::new();
                } else {
                    buf.push_str(line.as_str());
                }
            }
            Err(error) => {
                panic!("error: {}", error);
            }
        }
    }

    if buf.len() > 0 {
        total += count_fn(&buf);
    }

    println!("Total: {}", total);
}
