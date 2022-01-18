pub fn run() {
    let input = include_str!("day14input");

    let mut reindeers: Vec<Reindeer> = input.lines().map(|l| {
        let split: Vec<&str> = l.split(" ").collect();
        let name = split[0];
        let speed = split[3].parse::<i32>().unwrap();
        println!("{} {}", split[6], split[13]);
        let flight_time = split[6].parse::<i32>().unwrap();
        let rest_time = split[13].parse::<i32>().unwrap();

        Reindeer {
            name: name.to_string(),
            speed: speed,
            flight_time: flight_time,
            rest_time: rest_time,
            curr_pos: 0,
            curr_rest_time: 0,
            curr_flight_time: flight_time,
            curr_points: 0
        }
    }).collect();

    pt1(reindeers.clone());
    pt2(reindeers);
}

fn pt1(mut reindeers: Vec<Reindeer>) {
    let loop_count = 2503;
    for i in 0..loop_count {
        for reinder in reindeers.iter_mut() {
            if reinder.curr_rest_time == 0 {
                reinder.curr_pos += reinder.speed;
                reinder.curr_flight_time -= 1;
                if reinder.curr_flight_time == 0 {
                    reinder.curr_flight_time = reinder.flight_time;
                    reinder.curr_rest_time = reinder.rest_time;
                }
            } else {
                reinder.curr_rest_time -= 1;
            }
        }
    }

    println!("Part 1: {}", reindeers.iter().map(|r| r.curr_pos).max().unwrap());
}

fn pt2(mut reindeers: Vec<Reindeer>) {
    let loop_count = 2503;
    for i in 0..loop_count {
        for reinder in reindeers.iter_mut() {
            if reinder.curr_rest_time == 0 {
                reinder.curr_pos += reinder.speed;
                reinder.curr_flight_time -= 1;
                if reinder.curr_flight_time == 0 {
                    reinder.curr_flight_time = reinder.flight_time;
                    reinder.curr_rest_time = reinder.rest_time;
                }
            } else {
                reinder.curr_rest_time -= 1;
            }
        }

        reindeers.sort_by_key(|r| r.curr_pos);
        reindeers.reverse();
        let max = reindeers[0].curr_pos;
        reindeers.iter_mut().filter(|r| r.curr_pos == max).for_each(|r| r.curr_points += 1);
    }

    println!("Part 2: {}", reindeers.iter().map(|r| r.curr_points).max().unwrap());
}

#[derive(Debug,Clone)]
struct Reindeer {
    name: String,
    speed: i32,
    flight_time: i32,
    rest_time: i32,
    curr_pos: i32,
    curr_rest_time: i32,
    curr_flight_time: i32,
    curr_points: i32
}