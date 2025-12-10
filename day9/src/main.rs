use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let input = get_input();
    dbg!(&input);
    part1(input);
}

fn part1(input: Vec<(i64, i64)>) {
    let combinations = input
        .iter()
        .tuple_combinations::<(&(i64, i64), &(i64, i64))>();

    let mut max_size = 0;
    for combination in combinations {
        let diff1 = combination.0.0.abs_diff(combination.1.0) + 1;
        let diff2 = combination.0.1.abs_diff(combination.1.1) + 1;
        let size = diff1 * diff2;

        if size > max_size {
            max_size = size;
        }
    }
    dbg!(max_size);
}

fn get_input() -> Vec<(i64, i64)> {
    let string = read_to_string("data.txt").unwrap();
    let lines = string.lines();
    let mut input = Vec::new();
    for line in lines {
        let mut splits = line.split(',');
        input.push((
            splits.next().unwrap().parse().unwrap(),
            splits.next().unwrap().parse().unwrap(),
        ));
    }

    input
}
