use std::collections::HashSet;
use std::str::Chars;

pub fn run() {
    let input: &str = include_str!("day3input");

    println!("{:?}", input);

    pt1(&input);
    pt2(&input);
}

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

fn traverse(iter: impl Iterator<Item=char>) -> HashSet<Point> {
    let origin = Point { x: 0, y: 0 };
    let (_, visited) = iter.fold((origin, HashSet::new()), |(curr, mut visited), ch| {
        let x = match ch {
            '>' => &curr.x + 1,
            '<' => &curr.x - 1,
            _ => (&curr.x).clone()
        };
        let y = match ch {
            '^' => &curr.y + 1,
            'v' => &curr.y - 1,
            _ => (&curr.y).clone()
        };
        visited.insert(curr);
        (Point { x, y }, visited)
    });
    visited
}

fn pt1(input: &str) {
    let iter = input.chars();
    println!("{}", traverse(iter).len());
}

fn pt2(input: &str) {
    let mut santa = traverse(input.chars().step_by(2));
    let robo = traverse(input.chars().skip(1).step_by(2));
    santa.extend(robo);

    println!("{}", santa.len());
}