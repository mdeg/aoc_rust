use itertools::rev;
use std::collections::HashSet;

pub fn run() {
    pt1("vzbxkghb");
    pt2("vzbxxyzz");
}

fn pt1(input: &str) {
    println!("Part 1: {}", solve(&input));
}

fn pt2(input: &str) {
    println!("Part 2: {}", solve(&input));
}

fn solve(input: &str) -> String {
    let mut password: Vec<u8> = input.as_bytes().to_vec();
    loop {
        for i in (0..password.len()).rev() {
            if password[i] == 122 {
                password[i] = 97u8
            } else {
                password[i] = password[i] + 1;
                break
            }
        }

        if validate(&password) {
            break
        }
    }
    std::str::from_utf8(&password).unwrap().to_string()
}

fn validate(password: &[u8]) -> bool {
    let mut has_straight = false;

    let mut found_pairs: HashSet<u8> = HashSet::new();
    let mut dirty_index: HashSet<usize> = HashSet::new();

    for mut i in 0..password.len() - 1 {
        if password[i] == 'i' as u8 || password[i] == 'o' as u8 || password[i] == 'l' as u8 {
            return false
        }

        if i + 2 < password.len() {
            if password[i] + 1 == password[i + 1] && password[i] + 2 == password[i + 2] {
                has_straight = true
            }
        }

        if i + 1 < password.len() && !dirty_index.contains(&i) {
            if password[i] == password[i + 1] {
                found_pairs.insert(password[i]);
                dirty_index.insert(i);
            }
        }
    }

    has_straight && found_pairs.len() >= 2
}