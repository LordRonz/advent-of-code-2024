use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn string_from_file(filename: impl AsRef<Path>) -> String {
    let f = BufReader::new(File::open(filename).unwrap());

    f.lines().map(|l| l.unwrap()).collect()
}

fn part_one(memory: &String) -> i64 {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let matches: Vec<(i64, i64)> = re
        .captures_iter(memory.as_str())
        .filter_map(|cap| {
            let left = cap.get(1)?.as_str().parse::<i64>().ok();
            let right = cap.get(2)?.as_str().parse::<i64>().ok();
            Some((left?, right?))
        })
        .collect();

    let mut sum = 0;

    for mul in matches.iter() {
        sum += mul.0 * mul.1;
    }
    sum
}

fn part_two(memory: &String) -> i64 {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();

    let pattern_instr = r"do(?:n't)?\(\)";
    let re_instr = Regex::new(pattern_instr).unwrap();

    let matches_with_positions_and_numbers: Vec<(usize, i64, i64)> = re
        .captures_iter(memory.as_str())
        .filter_map(|cap| {
            let start = cap.get(0)?.start();
            let left = cap.get(1)?.as_str().parse::<i64>().ok();
            let right = cap.get(2)?.as_str().parse::<i64>().ok();
            Some((start, left?, right?))
        })
        .collect();

    let matches_instr: Vec<(&str, usize)> = re_instr
        .find_iter(memory.as_str())
        .map(|mat| (mat.as_str(), mat.start()))
        .collect();

    let mut sum = 0;

    let mut instr_index = 0;

    for (start, left, right) in matches_with_positions_and_numbers {
        let mut nearest_instr = matches_instr[instr_index].1;
        while nearest_instr < start && instr_index < matches_instr.len() - 1 {
            instr_index += 1;
            nearest_instr = matches_instr[instr_index].1;
        }
        if nearest_instr > start && instr_index > 0 {
            let instr = matches_instr[instr_index - 1].0;
            if instr == "do()" {
                sum += left * right;
            }
        } else {
            sum += left * right;
        }
    }
    sum
}

pub fn run() {
    println!("Day 03:");
    let memory = string_from_file("./src/day03/input");

    let sum_mul = part_one(&memory);
    let sum_mul_with_instr = part_two(&memory);

    println!("Part 1: {}", sum_mul);
    println!("Part 2: {}", sum_mul_with_instr);
}
