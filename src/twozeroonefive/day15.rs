pub fn run() {
    let input = include_str!("day15input_test");

    let ingredients = input.lines().map(|line| {
        let split: Vec<String> = line.split(" ").map(|word| word.replace(",","").replace(":","")).collect();

        Ingredient {
            name: split[0].to_string(),
            capacity: split[2].parse::<i32>().unwrap(),
            durability: split[4].parse::<i32>().unwrap(),
            flavor: split[6].parse::<i32>().unwrap(),
            texture: split[8].parse::<i32>().unwrap(),
            calories: split[10].parse::<i32>().unwrap()
        }

    }).collect();

    pt1(&ingredients);
}

fn pt1(ingredients: &Vec<Ingredient>) {
    println!("{:?}", ingredients);
}

#[derive(Debug,Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}