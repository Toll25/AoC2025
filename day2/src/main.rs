use std::fs::read_to_string;

fn main() {
    let inputs = get_input();
    part1(inputs);
}

fn part1(input: Vec<(u64, u64)>) {
    let mut counter = 0;
    for pair in input {
        for id in pair.0..=pair.1 {
            for check_num in 0..=10_i32.pow((id.to_string().len() / 2) as u32) {
                let check_string = check_num.to_string();
                let id_string = id.to_string();
                let stripped_once = id_string.strip_prefix(&check_string);
                if stripped_once.is_none() {
                    continue;
                }
                let stripped_twice = stripped_once.unwrap().strip_prefix(&check_string);
                if stripped_twice.is_none() {
                    continue;
                }
                if stripped_twice.unwrap().is_empty() {
                    println!("Adding number: {id}");
                    counter += id;
                }
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
