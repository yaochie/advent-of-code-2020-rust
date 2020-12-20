use std::io;

fn day13a() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let arrival_time = line.trim().parse::<u64>().unwrap();

    line.clear();
    io::stdin().read_line(&mut line).unwrap();

    let bus_ids: Vec<_> = line
        .trim()
        .split(',')
        .filter(|id| *id != "x")
        .map(|id| id.parse::<u64>().unwrap())
        .collect();

    let remainders: Vec<_> = bus_ids.iter().map(|id| id - (arrival_time % id)).collect();
    let a = remainders
        .iter()
        .enumerate()
        .min_by_key(|(_, x)| *x)
        .unwrap();

    let bus_id = bus_ids[a.0] as u64;
    let remainder = a.1;
    println!(
        "bus: {}, waiting time: {}, ans: {}",
        bus_id,
        remainder,
        (bus_id as u64) * remainder
    );
}

fn extended_euclidean(a: &i128, b: &i128) -> (i128, i128) {
    // return quotients (s, t) such that s*a + t*b = gcd(a, b)
    if a < b {
        let (s, t) = extended_euclidean(b, a);
        return (t, s);
    }

    assert_eq!(a >= b, true);

    let mut r0 = *a;
    let mut r1 = *b;

    let mut s0 = 1;
    let mut s1 = 0;

    let mut t0 = 0;
    let mut t1 = 1;

    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 % r1;
        let s = s0 - q * s1;
        let t = t0 - q * t1;

        r0 = r1;
        s0 = s1;
        t0 = t1;
        r1 = r;
        s1 = s;
        t1 = t;
    }

    (s0, t0)
}

fn day13b() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let bus_ids: Vec<_> = buf
        .trim()
        .split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(t, id)| {
            let bus_id = id.parse::<i128>().unwrap();
            ((bus_id - (t as i128)).rem_euclid(bus_id), bus_id)
        })
        .collect();

    if bus_ids.len() == 1 {
        // Only one bus, so the earliest time is its ID.
        println!("Ans: {}", bus_ids[0].1);
        return;
    }

    // Chinese remainder theorem
    // use the construction as shown on the Wikipedia page.

    let (a, b) = (bus_ids[0].0, bus_ids[1].0);
    let (s, t) = extended_euclidean(&bus_ids[0].1, &bus_ids[1].1);

    let mut x = b * s * bus_ids[0].1 + a * t * bus_ids[1].1;
    let mut prod = bus_ids[0].1 * bus_ids[1].1;

    x = x.rem_euclid(prod);

    println!("s: {}, t: {}, {} {}", s, t, x, prod);

    for (remainder, bus_id) in bus_ids[2..].iter() {
        let (s, t) = extended_euclidean(&prod, bus_id);
        x = remainder * s * prod + (x % prod) * t * bus_id;

        prod *= bus_id;
        x = x.rem_euclid(prod);
        println!("s: {}, t: {}, {} {} {}", s, t, bus_id, x, prod);
    }

    println!("{} {}", x, prod);
}

pub fn day13(part_a: bool) {
    if part_a {
        day13a()
    } else {
        day13b()
    }
}
