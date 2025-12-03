use std::fs::read_to_string;

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<u32>>) {
    let mut counter = 0;
    for line in input {
        let mut max1 = 0;
        let mut max1_index = 0;
        let mut max2 = 0;

        for (index, digit) in line[..line.len() - 1].iter().enumerate() {
            if *digit > max1 {
                max1 = *digit;
                max1_index = index;
            }
        }
        for digit in &line[max1_index + 1..] {
            if digit > &max2 {
                max2 = *digit;
            }
        }
        println!("Digit 1: {max1}");
        println!("Digit 2: {max2}");

        counter += format!("{max1}{max2}").parse::<u32>().unwrap();
    }

    println!("{counter}");
}

fn part2(input: &Vec<Vec<u32>>) {
    let mut counter = 0;
    for line in input {
        let mut numbers: Vec<(usize, u32)> = Vec::new();

        println!("{line:?}");
        for num in 0..12 {
            numbers.push((0, 0));
            let lower_limit = if num == 0 { 0 } else { numbers[num - 1].0 + 1 };
            let upper_limit = line.len() - 11 + num;
            // println!("lower_limit {lower_limit}");
            // println!("upper_limit {upper_limit}");
            for (index, digit) in line[lower_limit..upper_limit].iter().enumerate() {
                if *digit > numbers[num].1 {
                    numbers[num].1 = *digit;
                    numbers[num].0 = index + lower_limit;
                }
            }
            // println!("Digit: {}", numbers[num].1);
            // println!("Index: {}", numbers[num].0);
        }
        counter += numbers
            .iter()
            .map(|x| x.1.to_string())
            .collect::<String>()
            .parse::<u64>()
            .unwrap();

        // println!("{:?}", numbers.iter().map(|x| x.1).collect::<Vec<u32>>());
        // dbg!(numbers);
    }
    dbg!(counter);
}

fn get_input() -> Vec<Vec<u32>> {
    let lines: Vec<String> = read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut input = Vec::new();

    for line in lines {
        input.push(line.chars().map(|x| x.to_digit(10).unwrap()).collect());
    }

    input
}
