use std::io;
use std::io::Read;

const MODULO: u64 = 20201227;

/// Given a subject number and a resulting key, find
/// the loop size used to obtain the key.
fn get_loop_size(subject: u64, key: u64) -> u64 {
    let mut value = 1u64;
    let mut iter = 0u64;

    while value != key {
        value *= subject;
        value %= MODULO;
        iter += 1;

        // println!("after {} iterations, value is {}", iter, value);
    }

    iter
}

/// Given a subject number and a loop size, compute the
/// resulting key.
fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1u64;

    for _ in 0..loop_size {
        value *= subject;
        value %= MODULO;
    }

    value
}

pub fn day25() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read");

    let keys: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let key1 = keys[0];
    let key2 = keys[1];

    println!("public keys: {} {}", key1, key2);

    let loop_size1 = get_loop_size(7, key1);
    let loop_size2 = get_loop_size(7, key2);

    println!("loop sizes: {} {}", loop_size1, loop_size2);

    let enc_key1 = transform(key2, loop_size1);
    let enc_key2 = transform(key1, loop_size2);

    println!("encryption keys: {} {}", enc_key1, enc_key2);
    assert_eq!(enc_key1, enc_key2);
}
