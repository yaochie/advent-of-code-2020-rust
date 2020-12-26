use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

enum Rule {
    Char(char),
    Seq(Vec<u32>),
    Choice(Vec<u32>, Vec<u32>),
}

type Rules = HashMap<u32, Rule>;

fn parse_rule(string: &String) -> (u32, Rule) {
    let char_re = Regex::new(r#"^([0-9]+): "([a-z])"$"#).unwrap();
    let seq_re = Regex::new(r"^([0-9]+): ([0-9]+(?: [0-9]+)*)$").unwrap();
    let choice_re =
        Regex::new(r"^([0-9]+): ([0-9]+(?: [0-9]+)*) \| ([0-9]+(?: [0-9]+)*)$").unwrap();

    match (
        char_re.is_match(string),
        seq_re.is_match(string),
        choice_re.is_match(string),
    ) {
        (true, _, _) => {
            let caps = char_re.captures(string).unwrap();
            let id = caps[1].parse::<u32>().unwrap();
            let c = caps[2].chars().nth(0).unwrap();
            (id, Rule::Char(c))
        }
        (_, true, _) => {
            let caps = seq_re.captures(string).unwrap();
            let id = caps[1].parse::<u32>().unwrap();
            let values: Vec<u32> = caps[2]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            // println!("seq with {} values", values.len());
            (id, Rule::Seq(values))
        }
        (_, _, true) => {
            let caps = choice_re.captures(string).unwrap();
            let id = match caps[1].parse::<u32>() {
                Ok(n) => n,
                Err(_) => panic!("could not parse {}", caps[0].to_string()),
            };
            let choice1: Vec<u32> = caps[2]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let choice2: Vec<u32> = caps[3]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            // println!("choices: {} and {}", choice1.len(), choice2.len());
            (id, Rule::Choice(choice1, choice2))
        }
        _ => panic!("could not match {}", string),
    }
}

fn read_rules() -> Rules {
    let mut rules = HashMap::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                if line.trim().len() == 0 {
                    break;
                }

                let (id, rule) = parse_rule(&line.trim().to_string());
                rules.insert(id, rule);
            }
        }
    }
    println!("read {} rules", rules.len());

    rules
}

fn validate_rule(message: &String, rules: &Rules, rule_id: &u32, pos: usize) -> Option<usize> {
    match &rules[&rule_id] {
        Rule::Char(c) => {
            if *c == message.chars().nth(pos).unwrap() {
                Some(pos + 1)
            } else {
                None
            }
        }
        Rule::Seq(rule_seq) => {
            let mut next_pos = pos;
            for rule in rule_seq {
                match validate_rule(message, rules, rule, next_pos) {
                    Some(p) => next_pos = p,
                    None => return None,
                }
            }
            Some(next_pos)
        }
        Rule::Choice(rule_seq1, rule_seq2) => {
            let mut next_pos = pos;
            let mut match_seq1 = true;
            for rule in rule_seq1 {
                match validate_rule(message, rules, rule, next_pos) {
                    Some(p) => next_pos = p,
                    None => {
                        match_seq1 = false;
                        break;
                    }
                }
            }
            // how to detect if both seq1 and seq2 match, but the rest doesn't
            // match if seq1 is taken?
            // **guess it doesn't matter here, since both sequences are guaranteed
            // to consume the same number of characters since all choices are
            // balanced.

            if match_seq1 {
                return Some(next_pos);
            }

            let mut next_pos = pos;
            for rule in rule_seq2 {
                match validate_rule(message, rules, rule, next_pos) {
                    Some(p) => next_pos = p,
                    None => return None,
                }
            }
            Some(next_pos)
        }
    }
}

/// validate using recursive descent?
fn is_valid(message: &String, rules: &Rules) -> bool {
    if let Some(pos) = validate_rule(message, rules, &0, 0) {
        pos == message.len()
    } else {
        false
    }
}

fn combine_seq(seq: &Vec<HashSet<String>>) -> HashSet<String> {
    let mut result = HashSet::new();

    if seq.len() == 1 {
        result = seq[0].clone();
    } else if seq.len() == 2 {
        for seq1 in &seq[0] {
            for seq2 in &seq[1] {
                result.insert(format!("{}{}", seq1, seq2));
            }
        }
    } else {
        panic!();
    }

    result
}

fn get_possible_strings(rules: &Rules, rule_id: &u32) -> HashSet<String> {
    if *rule_id == 8 || *rule_id == 11 {
        panic!("rule {} is infinite!", rule_id);
    }

    match &rules[rule_id] {
        Rule::Char(c) => {
            let s = c.to_string();
            let mut result = HashSet::new();
            result.insert(s);
            result
        }
        Rule::Seq(rule_seq) => {
            let x: Vec<HashSet<String>> = rule_seq
                .iter()
                .map(|id| get_possible_strings(rules, id))
                .collect();
            combine_seq(&x)
        }
        Rule::Choice(rule_seq1, rule_seq2) => {
            let x1 = rule_seq1
                .iter()
                .map(|id| get_possible_strings(rules, id))
                .collect();
            let x2 = rule_seq2
                .iter()
                .map(|id| get_possible_strings(rules, id))
                .collect();

            let seq1 = combine_seq(&x1);
            let seq2 = combine_seq(&x2);

            let mut result = HashSet::new();
            for s in seq1.union(&seq2) {
                result.insert(s.clone());
            }
            result
        }
    }
}

fn is_valid_partb(message: &String, rules: &Rules) -> bool {
    // 0 -> 8 11
    // 8 -> 42 | 42 8
    // 11 -> 42 31 | 42 11 31
    // so we keep matching 42s, then match 31s
    // these are the only loops - no other rule goes to 8 or 11
    // also needs to have at least 2 42 matches, and 31 must be less than 42.

    let mut pos = 0;
    let mut num_match_42 = 0;
    while pos < message.len() {
        match validate_rule(message, rules, &42, pos) {
            Some(next_pos) => {
                num_match_42 += 1;
                pos = next_pos;
            }
            None => break,
        }
    }

    if num_match_42 < 2 || pos >= message.len() {
        return false;
    }

    let mut num_match_31 = 0;
    while pos < message.len() {
        match validate_rule(message, rules, &31, pos) {
            Some(next_pos) => {
                num_match_31 += 1;
                pos = next_pos;
            }
            None => return false,
        }
    }

    num_match_31 < num_match_42 && pos == message.len()
}

fn read_messages(rules: &Rules, part_a: bool) {
    let is_valid_fn = if part_a { is_valid } else { is_valid_partb };

    let mut n_valid = 0;
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(0) => break,
            Ok(_) => {
                let message = line.trim();

                if is_valid_fn(&message.to_string(), rules) {
                    println!("valid:   {}", message);
                    n_valid += 1;
                } else {
                    println!("invalid: {}", message);
                }
            }
        }
    }
    println!("# valid: {}", n_valid);
}

pub fn day19(part_a: bool) {
    let rules = read_rules();

    let match_42 = get_possible_strings(&rules, &42);
    let match_31 = get_possible_strings(&rules, &31);
    // println!("{}", match_42.intersection(&match_31).len());
    let n_match: i32 = match_42.intersection(&match_31).map(|_| 1).sum();
    assert_eq!(n_match, 0);
    println!("strings that match 42 & 31: {}", n_match);

    read_messages(&rules, part_a);
}
