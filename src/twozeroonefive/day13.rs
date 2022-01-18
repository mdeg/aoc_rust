use std::cmp::max;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("day13input");

    let grouped: Vec<(String, (String, i32))> = input.lines().map(|l| {
        let split: Vec<&str> = l.split(" ").collect();
        let name = split[0];

        let mut value = split[3].parse::<i32>().unwrap();
        if split[2] == "lose" {
            value = -value
        }

        let neighbour = split[10];

        (name.to_string(), (neighbour[0..neighbour.len() - 1].to_string(), value))
    }).collect();

    for i in 0..5 {
        let mut seats = HashMap::new();
        for item in grouped.clone() {
            let value = seats.entry(item.0).or_insert(vec!());
            value.push(item.1);
        }

        // println!("{:?}", seats);

        pt1(&seats);
        pt2(&mut seats.clone());
    }
    // pt2(&mut seats);
}

fn pt1(seats: &Seats) {
    let mut arr: Vec<String> = seats.keys().map(|k| k.to_string()).collect();
    let result = permute(arr.len(), arr, seats);

    println!("Part 1: {}", result);
}

fn pt2(seats: &mut Seats) {
    let mut arr: Vec<String> = seats.keys().map(|k| k.to_string()).collect();
    let mut first_result = 0;
    for perm in arr.clone().into_iter().permutations(arr.len()).unique() {
        first_result = max(first_result, score(perm, &seats));
    }
    // let first_result = permute(arr.len(), arr.clone(), seats);

    let relations = arr.iter().map(|name| (name.clone(), 0)).collect();
    for (_, neighbs) in seats.iter_mut() {
        neighbs.push(("Me".to_string(), 0));
    }
    seats.insert(String::from("Me"), relations);
    arr.push("Me".to_string());

    // let second_result = permute(arr.len(), arr, seats);

    let mut second_result = 0;
    for perm in arr.clone().into_iter().permutations(arr.len()).unique() {
        second_result = max(second_result, score(perm, &seats));
    }
    // println!("real {}", second_max);

    println!("Part 2: {} - {} = {}", first_result, second_result, (first_result - second_result).abs());
}

fn permute(k: usize, arr: Vec<String>, seats: &Seats) -> i32 {
    return if k == 1 {
        score(arr, seats)
    } else {
        let mut curr_max = permute(k - 1, arr.clone(), seats);
        for i in 0..k - 1 {
            let mut swappy = arr.clone();
            if k % 2 == 0 {
                swappy.swap(i, k - 1);
            } else {
                swappy.swap(0, k - 1);
            }
            curr_max = max(curr_max, permute(k - 1, swappy, seats));
        }
        curr_max
    }
}

type Seats = HashMap<String, Vec<(String, i32)>>;

fn score(item: Vec<String>, seats: &Seats) -> i32 {
    let mut sum = 0;
    for i in 0..item.len() {
        let right_neighbour = if i == item.len() - 1 {
            &item[0]
        } else {
            &item[i + 1]
        };
        let left_neighbour = if i == 0 {
            &item[item.len() - 1]
        } else {
            &item[i - 1]
        };
        let right_neighbours: Vec<&i32> = seats[&item[i]].iter()
            .filter(|(seat, value)| right_neighbour == seat)
            .map(|(_, v)| v)
            .take(1)
            .collect();

        let left_neighbours: Vec<&i32> = seats[&item[i]].iter()
            .filter(|(seat, value)| left_neighbour == seat)
            .map(|(_, v)| v)
            .take(1)
            .collect();

        sum += right_neighbours[0] + left_neighbours[0]
    }

    return sum
}