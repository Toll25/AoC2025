use std::fs::read_to_string;

fn main() {
    let inputs = get_input();
    part1(inputs.clone());
    part2(inputs);
}

fn part1(input: Vec<i32>) {
    let mut pointer = 50;
    let mut counter = 0;
    for number in input {
        let stripped_number = number % 100;
        // println!("Stripped Number: {stripped_number}");
        // println!("Pointer: {pointer}");
        if -stripped_number > pointer {
            pointer = 100 + pointer + stripped_number;
        } else {
            pointer += stripped_number;
            pointer %= 100;
        }
        // println!("New pointer: {pointer}");
        if pointer == 0 {
            counter += 1;
        }
    }
    println!("{counter}");
}

fn part2(input: Vec<i32>) {
    let mut pointer = 50;
    let mut counter = 0;
    for number in input {
        let stripped_number = number % 100;
        println!("Stripped Number: {stripped_number}");
        println!("Pointer: {pointer}");
        let num_total_rotations = number.abs() / 100;
        if num_total_rotations != 0 {
            println!("Increased counter (transitive): {counter} + {num_total_rotations}");
            counter += num_total_rotations;
        }
        if -stripped_number > pointer {
            // Equivalent to pointer + stripped_number < 0 but I only realized that after the fact
            if pointer != 0 {
                println!("Increased counter (over negative): {counter} + 1");
                counter += 1;
            }
            pointer = 100 + pointer + stripped_number;
        } else {
            if pointer + stripped_number > 100 {
                println!("Increased counter (over positive): {counter} + 1");
                counter += 1;
            }
            pointer += stripped_number;
            pointer %= 100;
        }
        println!("New pointer: {pointer}");
        if pointer == 0 {
            println!("Increased counter (final): {counter} + {0}", 1);
            counter += 1;
        }
        println!();
    }
    println!("{counter}");
}

fn get_input() -> Vec<i32> {
    let lines: Vec<String> = read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect();

    let mut inputs = Vec::new();
    for line in lines {
        let mut stripped_line = line.chars();
        stripped_line.next();

        let number: i32 = stripped_line.as_str().parse().unwrap();
        if line.starts_with('L') {
            inputs.push(-number);
        } else {
            inputs.push(number);
        }
    }

    inputs
}
