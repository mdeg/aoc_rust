type Sue = (i32, Vec<(String, i32)>);

pub fn run() {
    let input = include_str!("day16input");

    let mut sue = 0;
    let sues: Vec<Sue> = input.lines().map(|line| {
        let split: Vec<String> = line.split(" ")
            .map(|str| {
                if str.len() > 1 {
                    String::from(&str[0..str.len() - 1])
                } else {
                    String::from(&str[0..str.len()])
                }
            })
            .collect();

        sue += 1;
        (
            sue,
            vec!(
                (split[2].to_string(), split[3].parse::<i32>().unwrap()),
                (split[4].to_string(), split[5].parse::<i32>().unwrap()),
                (split[6].to_string(), split[7].parse::<i32>().unwrap())
            )
        )
    }).collect();

    pt1(&sues);
    pt2(&sues);
}

fn pt1(sues: &Vec<Sue>) {
    let matching_sues: Vec<i32> = sues.iter()
        .filter(|sue| {
            let gifts = &sue.1;
            for (gift, amount) in gifts {
                let count = match gift.as_str() {
                    "children" => 3,
                    "cats" => 7,
                    "samoyeds" => 2,
                    "pomeranians" => 3,
                    "akitas" => 0,
                    "vizslas" => 0,
                    "goldfish" => 5,
                    "trees" => 3,
                    "cars" => 2,
                    "perfumes" => 1,
                    _ => -1
                };

                if count != *amount {
                    return false
                }
            }
            return true
        })
        .map(|sue| sue.0)
        .collect();

    println!("Part 1: {:?}", matching_sues[0]);
}

fn pt2(sues: &Vec<Sue>) {
    let matching_sues: Vec<i32> = sues.iter()
        .filter(|sue| {
            let gifts = &sue.1;
            for (gift, amount) in gifts {
                let name = gift.as_str();
                let count = match name {
                    "children" => 3,
                    "cats" => 7,
                    "samoyeds" => 2,
                    "pomeranians" => 3,
                    "akitas" => 0,
                    "vizslas" => 0,
                    "goldfish" => 5,
                    "trees" => 3,
                    "cars" => 2,
                    "perfumes" => 1,
                    _ => -1
                };

                if name == "cats" || name == "trees" {
                    if count > *amount {
                        return false
                    }
                } else if name == "pomeranians" || name == "goldfish" {
                    if count < *amount {
                        return false
                    }
                } else {
                    if count != *amount {
                        return false
                    }
                }
            }
            return true
        })
        .map(|sue| sue.0)
        .collect();

    let without_other_sue: Vec<&i32> = matching_sues.iter().filter(|sue| **sue != 40).collect();
    println!("Part 2: {:?}", without_other_sue[0]);
}