use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Input {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

fn main() {
    let input = get_input();
    part1(input.clone());
    part2(input.ranges);
}

fn part1(input: Input) {
    let mut counter = 0;
    'ids: for id in input.ids {
        for range in &input.ranges {
            if id >= range.0 && id <= range.1 {
                counter += 1;
                continue 'ids;
            }
        }
    }
    println!("{counter}");
}

fn part2(mut input: Vec<(u64, u64)>) {
    let mut counter: u64 = 0;
    let mut checked_ranges: Vec<(u64, u64)> = Vec::new();
    input.sort_unstable();
    dbg!(&input);
    'ranges: for range in input {
        let mut lower_number = range.0;
        let mut higher_number = range.1;

        for check_range in &checked_ranges {
            if lower_number >= check_range.0 && higher_number <= check_range.1 {
                continue 'ranges;
            } else if check_range.1 >= lower_number && check_range.1 <= higher_number {
                lower_number = check_range.1 + 1;
            } else if higher_number >= check_range.0 && lower_number <= check_range.0 {
                higher_number = check_range.0 - 1;
            }
        }

        println!("{lower_number}:{higher_number}");
        counter += higher_number - lower_number + 1;
        checked_ranges.push((lower_number, higher_number));
    }

    println!("{counter}");
}

fn get_input() -> Input {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let string = read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = string.lines().map(str::trim).collect();

    let mut range_mode = true;
    for line in lines {
        if line.is_empty() {
            range_mode = false;
            continue;
        }

        if range_mode {
            let pair = line.split_once('-').unwrap();
            ranges.push((pair.0.parse().unwrap(), pair.1.parse().unwrap()));
        } else {
            ids.push(line.parse().unwrap());
        }
    }

    Input { ranges, ids }
}
