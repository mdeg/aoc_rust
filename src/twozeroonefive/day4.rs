use md5;
use std::fmt;

pub fn run() {
    let input: &str = "bgvyzdsv";

    println!("{}", find_nonce(&input, 5));
    println!("{}", find_nonce(&input, 6));
}

fn find_nonce(input: &str, count: i8) -> i32 {
    let mut i = 1;
    loop {
        let to_hash = format!("{}{}", input, i);
        let md5 = format!("{:x}", md5::compute(to_hash));
        let array: String = vec!['0'; count as usize].into_iter().collect();
        if &md5[0..count as usize] == &array {
            return i
        }
        i += 1;
    }
}