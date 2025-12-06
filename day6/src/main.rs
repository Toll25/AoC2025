use std::fs::read_to_string;

enum Operator {
    Add,
    Multiply,
}

fn main() {
    let input = get_input();
    part1(input.0.clone(), input.1.clone());
    part2(input.0, input.1);
}

fn part1(lines: Vec<String>, split_points: Vec<usize>) {
    let mut input = Vec::new();
    let mut last_split_point = 0;
    for split_point in &split_points {
        let mut nums = Vec::new();
        let mut operator = None;
        for line in &lines {
            let string = line[last_split_point..*split_point].trim();
            match string {
                "+" => operator = Some(Operator::Add),
                "*" => operator = Some(Operator::Multiply),
                _ => {
                    nums.push(string.parse::<u64>().unwrap());
                }
            }
        }
        last_split_point = *split_point;

        if let Some(unwrapped_operator) = operator {
            input.push((nums, unwrapped_operator));
        } else {
            unreachable!();
        }
    }

    dbg!(split_points);

    calculate(input);
}

fn part2(lines: Vec<String>, split_points: Vec<usize>) {
    let mut input = Vec::new();
    let mut last_split_point = 0;
    for split_point in &split_points {
        let mut operator = None;
        let mut num_strings = vec![String::new(); *split_point - last_split_point];
        for line in &lines {
            for index in last_split_point..*split_point {
                match line.chars().nth(index).unwrap() {
                    '+' => operator = Some(Operator::Add),
                    '*' => operator = Some(Operator::Multiply),
                    ' ' => {}
                    _ => {
                        num_strings[index - last_split_point] +=
                            &line.chars().nth(index).unwrap().to_string();
                    }
                }
            }
        }
        last_split_point = *split_point + 1;
        dbg!(&num_strings);
        if let Some(unwrapped_operator) = operator {
            input.push((
                num_strings.iter().map(|x| x.parse().unwrap()).collect(),
                unwrapped_operator,
            ));
        } else {
            unreachable!();
        }
    }

    dbg!(split_points);

    calculate(input);
}

fn calculate(input: Vec<(Vec<u64>, Operator)>) {
    let mut counter: u64 = 0;
    for col in input {
        counter += match col.1 {
            Operator::Add => col.0.iter().sum::<u64>(),
            Operator::Multiply => col.0.iter().product(),
        }
    }
    println!("{counter}");
}

fn get_input() -> (Vec<String>, Vec<usize>) {
    let string = read_to_string("data.txt").unwrap();
    let lines: Vec<String> = string
        .lines()
        .map(std::string::ToString::to_string)
        .collect();
    let mut split_points = Vec::new();
    for (col_index, _char) in lines[0].chars().enumerate() {
        let mut col_is_empty = true;
        for (row_index, _item) in lines.iter().enumerate() {
            if lines[row_index].chars().nth(col_index).unwrap() != ' ' {
                col_is_empty = false;
            }
        }
        if col_is_empty {
            split_points.push(col_index);
        }
    }

    split_points.push(lines[0].len());

    (lines, split_points)
}
