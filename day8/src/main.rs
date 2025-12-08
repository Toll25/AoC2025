use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;
use ordered_float::OrderedFloat;

fn main() {
    let input = get_input();
    dbg!(&input);
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<(i32, i32, i32)>) {
    let mut combinations = Vec::new();
    for (a, b) in (0..input.len()).tuple_combinations() {
        combinations.push((a, b, OrderedFloat(get_distance(input[a], input[b]))));
    }

    let mut network_map = HashMap::new();
    for el in input.iter().enumerate() {
        network_map.insert(el.0, el.0);
    }

    let mut combinations: Vec<(usize, usize)> = combinations
        .iter()
        .sorted_by_key(|x| x.2)
        .rev()
        .map(|x| (x.0, x.1))
        .collect();
    for _con_index in 0..1000 {
        let shortest = combinations.pop().unwrap();

        let first_network = network_map[&shortest.0];
        let second_network = network_map[&shortest.1];
        // networks.insert(shortest.1.1, Vec::new());
        let temp_network = network_map.clone();
        for second_network_box in temp_network.iter().filter(|x| *x.1 == second_network) {
            network_map.insert(*second_network_box.0, first_network);
        }
    }

    let mut network_size_map = HashMap::new();

    for entry in &network_map {
        network_size_map
            .entry(entry.1)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let mut sorted = network_size_map.iter().sorted_by_key(|x| x.1).rev();
    let num1 = sorted.next().unwrap().1;
    let num2 = sorted.next().unwrap().1;
    let num3 = sorted.next().unwrap().1;

    dbg!(num1 * num2 * num3);
}

fn part2(input: &Vec<(i32, i32, i32)>) {
    let mut combinations = Vec::new();
    for (a, b) in (0..input.len()).tuple_combinations() {
        combinations.push((a, b, OrderedFloat(get_distance(input[a], input[b]))));
    }

    let mut network_map = HashMap::new();
    for el in input.iter().enumerate() {
        network_map.insert(el.0, el.0);
    }
    let mut shortest = (0, 0);

    let mut combinations: Vec<(usize, usize)> = combinations
        .iter()
        .sorted_by_key(|x| x.2)
        .rev()
        .map(|x| (x.0, x.1))
        .collect();
    // dbg!(&combinations);
    while !network_map.values().all_equal() {
        shortest = combinations.pop().unwrap();
        // dbg!(&shortest);

        let first_network = network_map[&shortest.0];
        let second_network = network_map[&shortest.1];
        let second_network_boxes: Vec<usize> = network_map
            .iter()
            .filter(|x| *x.1 == second_network)
            .map(|x| *x.0)
            .collect();
        for second_network_box in second_network_boxes {
            network_map.insert(second_network_box, first_network);
        }
    }

    // dbg!(shortest);
    dbg!(input[shortest.0].0 * input[shortest.1].0);
}

fn get_distance(first: (i32, i32, i32), second: (i32, i32, i32)) -> f64 {
    let diff1 = f64::from(first.0 - second.0);
    let diff2 = f64::from(first.1 - second.1);
    let diff3 = f64::from(first.2 - second.2);
    f64::sqrt(f64::powi(diff1, 2) + f64::powi(diff2, 2) + f64::powi(diff3, 2))
}

fn get_input() -> Vec<(i32, i32, i32)> {
    let string = read_to_string("data.txt").unwrap();

    let lines = string.lines().map(str::trim);

    let mut input = Vec::new();
    for line in lines {
        let mut nums = line.split(',').map(|x| x.parse().unwrap());
        input.push((
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        ));
    }

    input
}
