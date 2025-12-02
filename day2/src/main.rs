use std::fs::read_to_string;

fn main() {
    let inputs = get_input();
    part1(inputs.clone());
    part2(inputs);
}

fn part1(input: Vec<(u64, u64)>) {
    let mut counter = 0;
    for pair in input {
        for id in pair.0..=pair.1 {
            let id_string = id.to_string();
            if id_string.len() % 2 != 0 {
                continue;
            }
            let halves = id_string.split_at(id_string.len() / 2);
            if halves.0 == halves.1 {
                counter += id;
            }
        }
    }
    println!("{counter}");
}

fn part2(input: Vec<(u64, u64)>) {
    let mut counter = 0;
    for pair in input {
        for id in pair.0..=pair.1 {
            let id_string = id.to_string();
            // println!("id_string {id_string}");
            let mut valid = false;
            for divisor in 1..=id_string.len() / 2 {
                if id_string.len() % divisor != 0 {
                    continue;
                }
                // println!("divisor {divisor}");
                let num_to_check = &id_string[0..divisor];
                // println!("num_to_check {num_to_check}");
                let num_parts = id_string.len() / divisor;
                // println!("num_parts {num_parts}");
                if num_parts < 2 {
                    break;
                }
                let mut divisor_valid = true;
                for part_num in 1..num_parts {
                    let part = &id_string[divisor * (part_num)..divisor * (part_num + 1)];
                    // println!("part{part_num} {part}");

                    if part != num_to_check {
                        divisor_valid = false;
                    }
                }
                if divisor_valid {
                    valid = true;
                }
            }
            if valid {
                println!("adding: {id}");
                counter += id;
            }
        }
    }
    println!("{counter}");
}

fn get_input() -> Vec<(u64, u64)> {
    let text: String = read_to_string("data.txt").unwrap().trim().to_string();
    let mut inputs = Vec::new();
    let pairs = text.split(',');
    for pair in pairs {
        let numbers = pair.split_once('-').unwrap();
        println!("{numbers:?}");
        inputs.push((numbers.0.parse().unwrap(), numbers.1.parse().unwrap()));
    }
    inputs
}
