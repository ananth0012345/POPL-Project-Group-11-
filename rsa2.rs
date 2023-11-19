use std::io;

fn main() {
    println!("ENTER FIRST PRIME NUMBER");
    let p: u64 = read_input();

    if !is_prime(p) {
        println!("WRONG INPUT");
        return;
    }

    println!("ENTER ANOTHER PRIME NUMBER");
    let q: u64 = read_input();

    if !is_prime(q) || p == q {
        println!("WRONG INPUT");
        return;
    }

    println!("ENTER MESSAGE");
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).expect("Failed to read line");

    let n = p * q;
    let t = (p - 1) * (q - 1);

    let (e, d) = ce(t, p, q);

    println!("POSSIBLE VALUES OF e AND d ARE");
    for i in 0..e.len() {
        println!("{}\t{}", e[i], d[i]);
    }

    let encrypted = encrypt(msg.trim(), e[0], n);
    let decrypted = decrypt(&encrypted, d[0], n);

    println!("THE ENCRYPTED MESSAGE IS\n{}", encrypted);
    println!("THE DECRYPTED MESSAGE IS\n{}", decrypted);
}

fn read_input() -> u64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

fn is_prime(pr: u64) -> bool {
    for i in 2..(pr as f64).sqrt() as u64 + 1 {
        if pr % i == 0 {
            return false;
        }
    }
    true
}

fn ce(t: u64, p: u64, q: u64) -> (Vec<u64>, Vec<u64>) {
    let mut e = Vec::new();
    let mut d = Vec::new();
    let mut k = 0;

    for i in 2..t {
        if t % i == 0 {
            continue;
        }
        if is_prime(i) && i != p && i != q {
            e.push(i);
            if let Some(flag) = cd(t, e[k]) {
                d.push(flag);
                k += 1;
            }
            if k == 99 {
                break;
            }
        }
    }

    (e, d)
}

fn cd(x: u64, t: u64) -> Option<u64> {
    let mut k = 1;

    loop {
        k += t;
        if k % x == 0 {
            return Some(k / x);
        }
    }
}

fn encrypt(msg: &str, key: u64, n: u64) -> String {
    let mut result = String::new();
    for ch in msg.chars() {
        let pt = (ch as u64) - 96;
        let mut k = 1;

        for _ in 0..key {
            k = (k * pt) % n;
        }

        let ct = k + 96;
        result.push((ct as u8) as char);
    }

    result
}

fn decrypt(encrypted: &str, key: u64, n: u64) -> String {
    let mut result = String::new();

    for ch in encrypted.chars() {
        let ct = (ch as u64) - 96;
        let mut k = 1;

        for _ in 0..key {
            k = (k * ct) % n;
        }

        let pt = k + 96;
        result.push((pt as u8) as char);
    }

    result
}
