use std::borrow::BorrowMut;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
    let input = include_str!("day9input");
    let split = input.lines().collect();

    pt1(&split);
    pt2(&split);

}

fn parse<'a>(input: &'a Vec<&str>) -> Locations<'a> {
    let mut locations: Locations = HashMap::new();

    for line in input {
        let reduced: Vec<&str> = line.split(" ").collect();
        let start = reduced[0];
        let end = reduced[2];
        let dist: i32 = reduced[4].parse().unwrap();

        locations.entry(start).or_insert(vec!()).push((end, dist));
        locations.entry(end).or_insert(vec!()).push((start, dist));
    }

    locations
}

type Locations<'a> = HashMap<&'a str, Vec<(&'a str, i32)>>;

fn pt1(input: &Vec<&str>) {
    let locations = parse(input);

    let min = locations.iter().map(|(start, links)| {
        traverse(&locations, start, vec!((start, 0)), false)
    })
        .map(|win| win.unwrap())
        .min()
        .unwrap();

    println!("Part 1: {}", min);
}

fn pt2(input: &Vec<&str>) {
    let locations = parse(input);

    let min = locations.iter().map(|(start, links)| {
        traverse(&locations, start, vec!((start, 0)), true)
    })
        .map(|win| win.unwrap())
        .min()
        .unwrap();

    println!("Part 2: {}", min);
}

fn traverse(locations: &Locations, curr: &str, path: Vec<(&str, i32)>, is_max: bool) -> Option<i32> {
    if locations.len() == path.len() {
        return Some(path.into_iter().map(|p| p.1).sum());
    }

    let mut min_or_max = None;
    if let Some(next_locs) = locations.get(curr) {
        let valid_next = next_locs.iter().filter(|next| !path.iter().any(|p| p.0 == next.0));
        for next in valid_next {
            let mut new_path = path.clone();
            new_path.push(*next);
            let result = traverse(locations, next.0, new_path, is_max);

            if let Some(num) = result {
                min_or_max = match min_or_max {
                    Some(curr) => {
                        if num < curr && !is_max {
                            Some(num)
                        } else if num > curr && is_max {
                            Some(num)
                        } else {
                            min_or_max
                        }
                    },
                    None => Some(num)
                }
            }
        }
    }

    return min_or_max
}