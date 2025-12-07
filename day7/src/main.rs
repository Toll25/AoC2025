use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Empty,
    Splitter,
}

#[derive(Debug, Clone)]
struct Input {
    map: Vec<Vec<Value>>,
    start: usize,
}

fn main() {
    let input = get_input();
    // dbg!(&input);
    part1(input.clone());
    part2(input);
}

fn part1(input: Input) {
    let mut counter = 0;
    let mut prev_beams: Vec<usize> = vec![input.start];
    for line in input.map {
        let mut beams: Vec<usize> = Vec::new();
        for beam in &prev_beams {
            match line[*beam] {
                Value::Splitter => {
                    beams.push(beam + 1);
                    beams.push(beam - 1);
                    counter += 1;
                }
                Value::Empty => {
                    beams.push(*beam);
                }
            }
        }
        prev_beams = beams.into_iter().unique().collect();
    }
    println!("{counter}");
}
fn part2(input: Input) {
    let reversed_map: Vec<Vec<Value>> = input.map.into_iter().rev().collect();
    // horizontal location and beam count
    let mut beams: HashMap<usize, i32> = HashMap::new();
    for index in 0..reversed_map[0].len() {
        beams.insert(index, 1);
    }
    for line in reversed_map {
        let mut new_beams = HashMap::new();

        for beam in beams {
            let left = if beam.0 == 0 {
                Value::Empty
            } else {
                line[beam.0 - 1].clone()
            };

            let right = if beam.0 == line.len() - 1 {
                Value::Empty
            } else {
                line[beam.0 + 1].clone()
            };

            if left == Value::Splitter || right == Value::Splitter {
                if left == Value::Splitter {
                    new_beams.insert(beam.0 - 1, beam.1 + 1);
                }
                if right == Value::Splitter {
                    new_beams.insert(beam.0 + 1, beam.1 + 1);
                }
            } else if line[beam.0] == Value::Empty {
                new_beams.insert(beam.0, beam.1);
            }
        }

        beams = new_beams;
    }

    dbg!(&beams);

    let mut timeline_counter = 0;
    for beam in beams {
        if beam.0 == input.start {
            timeline_counter += beam.1;
        }
    }
    println!("{timeline_counter}");
}

fn get_input() -> Input {
    let string = read_to_string("test.txt").unwrap();
    let lines: Vec<&str> = string.lines().map(str::trim).collect();
    let start = lines[0].find('S').unwrap();
    let map = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '^' => Value::Splitter,
                    _ => Value::Empty,
                })
                .collect()
        })
        .collect();

    Input { map, start }
}
