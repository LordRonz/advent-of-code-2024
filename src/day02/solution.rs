use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<i64>> {
    let f = BufReader::new(File::open(filename).unwrap());

    f.lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(line: &Vec<i64>) -> bool {
    let mut is_safe = true;
    let mut dir = 0;
    for report_pair in line.windows(2) {
        let diff = report_pair[0] - report_pair[1];
        let new_dir = if diff > 0 {
            1
        } else if diff < 0 {
            -1
        } else {
            0
        };

        if diff == 0 || (dir != 0 && dir != new_dir) || diff.abs() > 3 {
            is_safe = false;
            break;
        }
        dir = new_dir;
    }
    is_safe
}

fn part_two(lines: &Vec<Vec<i64>>) -> u64 {
    let mut safe_count = 0;
    for reports in lines.iter() {
        for i in 0..reports.len() {
            let tolerated_levels: Vec<_> = reports[..i]
                .iter()
                .chain(&reports[i + 1..])
                .cloned()
                .collect();

            if is_safe(&tolerated_levels) {
                safe_count += 1;
                break;
            }
        }
    }
    safe_count
}

fn part_one(lines: &Vec<Vec<i64>>) -> u64 {
    let mut safe_count = 0;
    for line in lines.iter() {
        if is_safe(&line) {
            safe_count += 1;
        }
    }
    safe_count
}

pub fn run() {
    println!("Day 02:");
    let lines = lines_from_file("./src/day02/input");
    // Part A
    let safe_count_1 = part_one(&lines);
    let safe_count_2 = part_two(&lines);

    println!("Part 1: {}", safe_count_1);
    println!("Part 2: {}", safe_count_2);
}
