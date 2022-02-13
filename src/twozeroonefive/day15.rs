use std::cmp::max;

pub fn run() {
    let input = include_str!("day15input");

    let ingredients = input.lines().map(|line| {
        let split: Vec<String> = line.split(" ").map(|word| word.replace(",","").replace(":","")).collect();

        Ingredient {
            name: split[0].to_string(),
            capacity: split[2].parse::<i64>().unwrap(),
            durability: split[4].parse::<i64>().unwrap(),
            flavor: split[6].parse::<i64>().unwrap(),
            texture: split[8].parse::<i64>().unwrap(),
            calories: split[10].parse::<i64>().unwrap()
        }

    }).collect();

    pt1(&ingredients);
    pt2(&ingredients);
}

fn pt1(ingredients: &Vec<Ingredient>) {
    let mut best = -1;

    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                for l in 0..=(100 - i - j - k) {
                    if i + j + k + l == 100 {
                        let quantity = vec!(i,j,k,l);
                        let result = calc_value(ingredients, &quantity);
                        if result > best {
                            best = result
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", best);
}

fn pt2(ingredients: &Vec<Ingredient>) {
    let mut best = -1;

    for i in 0..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - i - j) {
                for l in 0..=(100 - i - j - k) {
                    if i + j + k + l == 100 {
                        let quantity = vec!(i,j,k,l);
                        let result = calc_value(ingredients, &quantity);
                        let calories = calc_calories(ingredients, &quantity);

                        if result > best && calories == 500 {
                            best = result
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", best);
}

fn calc_value(ingredients: &Vec<Ingredient>, quantities: &Vec<i64>) -> i64 {
    let value = quantities.iter()
        .zip(ingredients)
        .map(|(quant, ingredient)| {
            (
                (ingredient.capacity * quant),
                (ingredient.durability * quant),
                (ingredient.flavor * quant),
                (ingredient.texture * quant)
            )
        })
        .fold((0, 0, 0, 0), |acc, x| {
            (
                acc.0 + x.0,
                acc.1 + x.1,
                acc.2 + x.2,
                acc.3 + x.3
            )
        });

    max(0, value.0) * max(0, value.1) * max(0, value.2) * max(0, value.3)
}

fn calc_calories(ingredients: &Vec<Ingredient>, quantities: &Vec<i64>) -> i64 {
    quantities.iter()
        .zip(ingredients)
        .map(|(quant, ingredient)| ingredient.calories * quant )
        .fold(0, |acc, x| acc + x)
}

#[derive(Debug,Clone)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}