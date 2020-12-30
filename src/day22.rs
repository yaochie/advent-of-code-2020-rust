use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

type Deck = VecDeque<u32>;

/// Read cards. Terminates on blank line or EOF.
fn read_player() -> Deck {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error!");

    let mut cards = VecDeque::new();
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(error) => panic!("error: {}", error),
            Ok(n) => {
                if n == 0 || line.trim().len() == 0 {
                    break;
                }

                cards.push_back(line.trim().parse::<u32>().unwrap());
            }
        }
    }

    cards
}

fn compute_score(deck: &Deck) -> u32 {
    let mut score = 0;
    for (i, card) in deck.iter().rev().enumerate() {
        score += (i as u32 + 1) * card;
    }

    score
}

fn print_deck(deck: &Deck) {
    for card in deck {
        print!("{}, ", card);
    }
    println!();
}

fn play_game(p1_cards: &mut Deck, p2_cards: &mut Deck) {
    loop {
        if p1_cards.len() == 0 || p2_cards.len() == 0 {
            // game ends
            if p1_cards.len() == 0 {
                println!("Score: {}", compute_score(&p2_cards));
            } else {
                println!("Score: {}", compute_score(&p1_cards));
            }
            return;
        }

        let c1 = p1_cards.pop_front().unwrap();
        let c2 = p2_cards.pop_front().unwrap();

        match c1.cmp(&c2) {
            Ordering::Greater => {
                // player 1 wins round
                p1_cards.push_back(c1);
                p1_cards.push_back(c2);
            }
            Ordering::Less => {
                // player 2 wins round
                p2_cards.push_back(c2);
                p2_cards.push_back(c1);
            }
            Ordering::Equal => panic!("got same card in both decks"),
        }
    }
}

enum Player {
    Player1,
    Player2,
}

fn hash_game(p1_cards: &Deck, p2_cards: &Deck) -> String {
    let mut hash = String::new();

    for card in p1_cards {
        hash.push_str(format!("{},", card).as_str());
    }
    hash.push_str("|");
    for card in p2_cards {
        hash.push_str(format!("{},", card).as_str());
    }

    hash
}

fn play_recursive_game(p1_cards: &mut Deck, p2_cards: &mut Deck) -> Player {
    // keep a set of all rounds (as strings) to prevent infinite games
    let mut rounds = HashSet::new();

    loop {
        let hash = hash_game(p1_cards, p2_cards);
        if rounds.contains(&hash) {
            return Player::Player1;
        }

        rounds.insert(hash);

        match (p1_cards.len() == 0, p2_cards.len() == 0) {
            (true, false) => return Player::Player2,
            (false, true) => return Player::Player1,
            (true, true) => panic!("both players have no cards in their decks"),
            (false, false) => (),
        }

        let c1 = p1_cards.pop_front().unwrap();
        let c2 = p2_cards.pop_front().unwrap();

        // check if we go into a subgame
        if (c1 as usize) <= p1_cards.len() && (c2 as usize) <= p2_cards.len() {
            let mut p1_subdeck = VecDeque::new();
            for i in 0..c1 {
                p1_subdeck.push_back(p1_cards.get(i as usize).unwrap().clone());
            }

            let mut p2_subdeck = VecDeque::new();
            for i in 0..c2 {
                p2_subdeck.push_back(p2_cards.get(i as usize).unwrap().clone());
            }

            match play_recursive_game(&mut p1_subdeck, &mut p2_subdeck) {
                Player::Player1 => {
                    p1_cards.push_back(c1);
                    p1_cards.push_back(c2);
                }
                Player::Player2 => {
                    p2_cards.push_back(c2);
                    p2_cards.push_back(c1);
                }
            }
        } else {
            // play normally
            match c1.cmp(&c2) {
                Ordering::Greater => {
                    // player 1 wins round
                    p1_cards.push_back(c1);
                    p1_cards.push_back(c2);
                }
                Ordering::Less => {
                    // player 2 wins round
                    p2_cards.push_back(c2);
                    p2_cards.push_back(c1);
                }
                Ordering::Equal => panic!("got same card in both decks"),
            }
        }
    }
}

pub fn day22(part_a: bool) {
    let mut p1_cards = read_player();
    let mut p2_cards = read_player();

    print_deck(&p1_cards);
    print_deck(&p2_cards);

    if part_a {
        play_game(&mut p1_cards, &mut p2_cards);
    } else {
        let score = match play_recursive_game(&mut p1_cards, &mut p2_cards) {
            Player::Player1 => compute_score(&p1_cards),
            Player::Player2 => compute_score(&p2_cards),
        };

        println!("Score: {}", score);
    }
}
