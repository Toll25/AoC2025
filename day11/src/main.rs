use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let text = read_to_string("data.txt").unwrap();
    let input = get_input(&text);
    part1(&input);
    part2(&input);
}

fn part1(input: &HashMap<&str, Vec<&str>>) {
    let answer = recursion(input, "you");
    dbg!(answer);
}

fn part2(input: &HashMap<&str, Vec<&str>>) {
    let mut memo = HashMap::new();
    let answer = recursionpt2(input, "svr", false, false, &mut memo);
    dbg!(answer);
}

fn recursion(input: &HashMap<&str, Vec<&str>>, current: &str) -> i32 {
    if current == "out" {
        return 1;
    }
    let targets = input.get(current).unwrap();
    let mut sum = 0;
    for target in targets {
        sum += recursion(input, target);
    }
    sum
}

fn recursionpt2(
    input: &HashMap<&str, Vec<&str>>,
    current: &str,
    mut fft: bool,
    mut dac: bool,
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    if let Some(&cached) = memo.get(&(current.to_string(), fft, dac)) {
        return cached;
    }

    match current {
        "fft" => fft = true,
        "dac" => dac = true,
        "out" => {
            if fft && dac {
                return 1;
            }
            return 0;
        }
        _ => (),
    }
    let targets = input.get(current).unwrap();
    let mut sum = 0;
    for target in targets {
        sum += recursionpt2(input, target, fft, dac, memo);
    }
    memo.insert((current.to_string(), fft, dac), sum);

    sum
}

fn get_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();
    for line in input.lines() {
        let splits = line.split_once(':').unwrap();
        map.insert(splits.0.trim(), splits.1.trim().split(' ').collect());
    }

    map
}
