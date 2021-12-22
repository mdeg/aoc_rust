use std::str::from_utf8;

pub fn run() {
    let input = include_str!("day8input");
    let split = input.lines().collect();

    println!("--------pt1--------");
    pt1(&split);
    println!("--------pt2--------");
    pt2(&split);
}

fn compute(acc: (i32, i32), line: &[u8]) -> (i32, i32) {
    let mut index = 0;
    let (mut str_chars, mut total_chars) = (0, 0);
    while index < line.len() {
        let char = line[index] as char;
        match char {
            '\"' => {
                total_chars += 1;
                index += 1;
            },
            '\\' => {
                let next_char = line[index + 1] as char;
                match next_char {
                    '"' | '\\' => {
                        total_chars += 2;
                        str_chars += 1;
                        index += 2
                    },
                    'x' => {
                        total_chars += 4;
                        str_chars += 1;
                        index += 4
                    }
                    _ => {}
                }
            },
            _ => {
                str_chars += 1;
                total_chars += 1;
                index += 1;
            }
        }
    }
    (acc.0 + str_chars, acc.1 + total_chars)
}

fn pt1(input: &Vec<&str>) {
    let (str_chars, total_chars) = input.iter().map(|i| i.as_bytes()).fold((0, 0), compute);

    println!("String: {} Total: {}", str_chars, total_chars);
    println!("Sum: {}", total_chars - str_chars);
}

fn pt2(input: &Vec<&str>) {
    let (_, orig_chars) = input.iter().map(|i| i.as_bytes()).fold((0, 0), compute);

    let new_strings: Vec<Vec<u8>> = input.iter().map(|line| {
        let mut line = line.as_bytes().iter().fold(vec![b'\"'], |mut build, char| {
            match char {
                b'"' => build.extend(br#"\""#),
                b'\\' => build.extend(br#"\\"#),
                _ => build.push(*char)
            };
            build
        });
        line.push(b'\"');
        line
    }).collect();

    let (_, new_total_chars) = new_strings.iter().map(|i| i.as_slice()).fold((0, 0), compute);

    println!("{} - {} = {}", new_total_chars, orig_chars, new_total_chars - orig_chars);
}
