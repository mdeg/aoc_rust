use std::cmp::min;

pub fn run() {
    let input: Vec<Vec<i32>> = include_str!("day2input")
        .lines()
        .map(|l| { l.split("x").map(|i| i.parse::<i32>().unwrap()).collect() })
        .collect();

    pt1(&input);
    pt2(&input);
}

fn pt1(input: &Vec<Vec<i32>>) {
    let result: i32 = input.iter().map(|set| {
        let (l, w, h) = (set[0], set[1], set[2]);
        (2 * (l * w)) + (2 * (w * h)) + (2 * (h * l)) + min(min((l * w), (w * h)), (h * l))
    }).sum();
    println!("pt1: {}", result)
}

fn pt2(input: &Vec<Vec<i32>>) {
    let result: i32 = input.iter().map(|set| {
        let mut set = set.clone();
        set.sort();
        let ribbon = (set[0] * 2) + (set[1] * 2);
        let bow = (set[0] * set[1] * set[2]);
        ribbon + bow
    }).sum();
    println!("pt2: {}", result)
}