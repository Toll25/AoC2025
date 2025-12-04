use std::fs::read_to_string;

fn main() {
    let input = get_input();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Vec<bool>>) {
    let mut counter = 0;
    input.iter().enumerate().for_each(|x| {
        x.1.iter().enumerate().for_each(|y| {
            if *y.1 && check_surrounding(input, (x.0, y.0), 4) {
                println!("Valid Roll: {0} {1}", x.0, y.0);
                counter += 1;
            }
        });
    });
    println!("{counter}");
}

fn part2(input: &Vec<Vec<bool>>) {
    let mut counter = 0;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    let mut input = input.clone();
    loop {
        for entry in to_remove {
            input[entry.0][entry.1] = false;
        }
        to_remove = Vec::new();
        input.iter().enumerate().for_each(|x| {
            x.1.iter().enumerate().for_each(|y| {
                if *y.1 && check_surrounding(&input, (x.0, y.0), 4) {
                    // println!("Valid Roll: {0} {1}", x.0, y.0);
                    counter += 1;
                    to_remove.push((x.0, y.0));
                }
            });
        });
        if to_remove.is_empty() {
            break;
        }
    }
    println!("{counter}");
}

fn check_surrounding(map: &Vec<Vec<bool>>, pos: (usize, usize), limit: i32) -> bool {
    let mut counter = 0;

    let lower_line = pos.0.saturating_sub(1);
    let lower_value = pos.1.saturating_sub(1);
    for pos0 in lower_line..=pos.0 + 1 {
        for pos1 in lower_value..=pos.1 + 1 {
            if (pos0, pos1) == pos {
                continue;
            }
            if let Some(line) = map.get(pos0)
                && let Some(value) = line.get(pos1)
                && *value
            {
                // println!("Counting {pos0}:{pos1} for {}:{}", pos.0, pos.1);
                counter += 1;
            }
        }
    }

    counter < limit
}

fn get_input() -> Vec<Vec<bool>> {
    let text = read_to_string("data.txt").unwrap();

    text.lines()
        .map(|x| x.trim().chars().map(|x| x == '@').collect())
        .collect()
}
