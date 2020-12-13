use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::iter::FromIterator;

use regex::Regex;

fn day7a() {
    // we build a map from bags to bags that contain it.
    // then we start from the shiny gold bag and find all containers of it.

    let sub_bag = Regex::new(r"(\d+) (.*) bags?").unwrap();

    let mut bags: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let parts: Vec<&str> = line.trim().split(" contain ").collect();

                let name = parts[0].trim_end_matches(" bags");

                let contents: Vec<&str> = parts[1]
                    .trim_end_matches('.')
                    .split(',')
                    .map(|s| s.trim())
                    .collect();

                for content in contents {
                    if content.contains("no other") {
                        continue;
                    }

                    let bag_type = sub_bag.captures(content).unwrap().get(2).unwrap().as_str();

                    match bags.get_mut(bag_type) {
                        None => {
                            let x = HashSet::from_iter([name.to_string()].iter().cloned());
                            bags.insert(bag_type.to_string(), x);
                            ()
                        }
                        Some(containers) => {
                            containers.insert(name.to_string());
                            ()
                        }
                    }
                }
            }
            Err(error) => panic!("error: {}", error),
        }
    }

    let mut initial_set = match bags.get("shiny gold") {
        Some(containers) => containers.clone(),
        _ => panic!("didn't find shiny gold bags"),
    };

    loop {
        // for each bag in the set, find all bags that contain them and join.
        let new_set = initial_set.iter().map(|bag| bags.get(bag)).fold(
            initial_set.clone(),
            |acc, c| match c {
                Some(container) => acc.union(container).cloned().collect(),
                _ => acc,
            },
        );

        assert_eq!(initial_set.is_subset(&new_set), true);

        if new_set.len() == initial_set.len() {
            break;
        }

        initial_set = new_set;
    }

    println!("Number of possible bags: {}", initial_set.len());
}

fn day7b() {
    // we build a map from bags to their contents
    // then we start from the shiny gold bag and find all its contents
    // (as a graph search)

    let sub_bag = Regex::new(r"(\d+) (.*) bags?").unwrap();

    let mut bag_contents: HashMap<String, HashMap<String, i32>> = HashMap::new();

    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let parts: Vec<&str> = line.trim().split(" contain ").collect();

                let name = parts[0].trim_end_matches(" bags");

                if parts[1] == "no other bags." {
                    bag_contents.insert(name.to_string(), HashMap::new());
                    continue;
                }

                let contents: HashMap<_, _> = parts[1]
                    .trim_end_matches('.')
                    .split(',')
                    .map(|s| {
                        let caps = sub_bag.captures(s.trim()).unwrap();

                        let bag_count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                        let bag_type = caps.get(2).unwrap().as_str();

                        (bag_type.to_string(), bag_count)
                    })
                    .collect();

                bag_contents.insert(name.to_string(), contents);
            }
            Err(error) => panic!("error: {}", error),
        }
    }

    // depth-first search
    let mut total_bag_count = 0;
    let mut to_visit = VecDeque::new();
    to_visit.push_back((1, "shiny gold"));

    while to_visit.len() > 0 {
        let (count, bag) = to_visit.pop_back().unwrap();
        total_bag_count += count;

        let contents = bag_contents.get(bag).unwrap();
        for (bag_type, bag_count) in contents {
            to_visit.push_back((bag_count.clone() * count, bag_type));
        }
    }

    // subtract one for the initial shiny gold bag
    println!("Total number of bags: {}", total_bag_count - 1);
}

pub fn day7(part_a: bool) {
    if part_a {
        day7a()
    } else {
        day7b()
    }
}
