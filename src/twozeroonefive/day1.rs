pub fn run() {
    let input = include_str!("day1input");

    pt1(input);
    pt2(input);
}

fn pt1(input: &str) {
    let result = input.chars().fold(0, |acc, ch| {
        match ch {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        }
    });

    println!("{}", result)
}

fn pt2(input: &str) {
    let (_, result) = input.chars().enumerate().fold((0, None), |(acc, found), (i, ch)| {
        let found = match found {
            None => {
                if acc == -1 {
                    Some(i)
                } else {
                    None
                }
            },
            _ => found
        };
        let acc = match ch {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        };
        (acc, found)
    });

    println!("{:?}", result)
}