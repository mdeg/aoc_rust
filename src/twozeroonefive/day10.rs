pub fn run() {
    let input = "1113122113";

    pt1(&input);
    pt2(&input);
}

fn pt1(input: &str) {
    let loop_count = 40;
    println!("Part 1: {}", solve(input, loop_count));
}

fn pt2(input: &str) {
    let loop_count = 50;
    println!("Part 2: {}", solve(input, loop_count));
}

fn solve(input: &str, loop_count: i32) -> i32 {
    let mut curr: Vec<u8> = input.as_bytes().to_vec();
    println!("{:?}", curr);
    for _ in 0..loop_count {
        let mut next = vec!();
        let mut i = 0;
        while i < curr.len() {
            let curr_digit = curr[i];
            let mut count = 1;
            while i + 1 < curr.len() && curr_digit == curr[i + 1] {
                count += 1;
                i += 1;
            }
            //
            next.push(48 + count);
            next.push(curr_digit);

            i += 1
        }
        curr = next;
    }

    curr.len() as i32
}