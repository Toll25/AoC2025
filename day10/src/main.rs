use std::fs::read_to_string;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Machine {
    light_goal: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
}

fn main() {
    let input = get_input();
    // dbg!(&input);
    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<Machine>) {
    let mut total_counter = 0;
    'machines: for machine in input {
        let mut num_buttons = 0;

        loop {
            let combs = (0..machine.buttons.len()).combinations_with_replacement(num_buttons);
            for comb in combs {
                let mut lights = vec![false; machine.light_goal.len()];
                for button_id in comb {
                    for instruction in machine.buttons[button_id].clone() {
                        lights[instruction as usize] = !lights[instruction as usize];
                    }
                }
                if lights == machine.light_goal {
                    total_counter += num_buttons;
                    continue 'machines;
                }
            }
            num_buttons += 1;
        }
    }
    dbg!(total_counter);
}

fn part2(input: Vec<Machine>) {
    let mut total_counter = 0;
    'machines: for (index, machine) in input.iter().enumerate() {
        let mut num_buttons = 0;

        loop {
            let combs = (0..machine.buttons.len()).combinations_with_replacement(num_buttons);
            for comb in combs {
                let mut joltage_counter = vec![0; machine.joltage.len()];
                for button_id in comb {
                    for instruction in machine.buttons[button_id].clone() {
                        joltage_counter[instruction as usize] += 1;
                    }
                }
                if joltage_counter == machine.joltage {
                    total_counter += num_buttons;
                    println!("Finished MachineNr: {} out of {}", index + 1, input.len());
                    continue 'machines;
                }
            }
            num_buttons += 1;
        }
    }
    dbg!(total_counter);
}

fn get_input() -> Vec<Machine> {
    let mut machines = Vec::new();

    let string = read_to_string("test.txt").unwrap();
    let outer = Regex::new(r"\[(.*)\]( \(.*\)) \{(.*)\}").unwrap();
    let inner = Regex::new(r" \((.*?)\)").unwrap();
    for (_, [brackets, paren, curly]) in outer.captures_iter(&string).map(|c| c.extract()) {
        let light_goal = brackets
            .chars()
            .map(|x| match x {
                '.' => false,
                '#' => true,
                _ => unreachable!(),
            })
            .collect();
        let buttons = inner
            .captures_iter(paren)
            .map(|c| c.extract())
            .map(|x: (&str, [&str; 1])| string_to_num_vec(x.1[0]))
            .collect();
        let joltage = string_to_num_vec(curly);

        let machine = Machine {
            light_goal,
            buttons,
            joltage,
        };
        machines.push(machine);
    }

    machines
}

fn string_to_num_vec(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}
