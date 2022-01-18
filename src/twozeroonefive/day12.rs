use std::borrow::BorrowMut;
use serde_json::{Result, Value};

pub fn run() {
    let input = include_str!("day12input");

    pt1(&input);
    pt2(&input);
}

fn pt1(input: &str) {
    let value: Value = serde_json::from_str(input).unwrap();

    let mut values = vec!(value);

    let mut sum = 0;
    while !values.is_empty() {
        let curr = values.pop().unwrap();

        match curr {
            Value::Number(num) => {
                sum += num.as_i64().unwrap()
            },
            Value::Array(arr) => values.append(arr.clone().as_mut()),
            Value::Object(mut map) => {
                let mut x: Vec<Value> = map.values().cloned().collect();
                values.append(&mut x)
            },
            _ => {}
        }
    }

    println!("Part 1: {}", sum);
}

fn pt2(input: &str) {
    let value: Value = serde_json::from_str(input).unwrap();

    let mut values = vec!(value);

    let mut sum = 0;
    while !values.is_empty() {
        let curr = values.pop().unwrap();

        match curr {
            Value::Number(num) => {
                sum += num.as_i64().unwrap()
            },
            Value::Array(arr) => values.append(arr.clone().as_mut()),
            Value::Object(mut map) => {
                if !map.values().any(|v| v == "red") {
                    let mut x: Vec<Value> = map.values().cloned().collect();
                    values.append(&mut x)
                }
            },
            _ => {}
        }
    }

    println!("Part 1: {}", sum);
}