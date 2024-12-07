use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<Vec<i64>> {
    let f = BufReader::new(File::open(filename).unwrap());

    f.lines()
        .map(|l| {
            l.unwrap()
                .split("   ")
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(vec_a: &Vec<i64>, vec_b: &Vec<i64>) -> u64 {
    let mut sum: u64 = 0;

    for (a, b) in vec_a.iter().zip(vec_b.iter()) {
        sum += a.abs_diff(*b);
    }

    return sum;
}

fn part_two(vec_a: &Vec<i64>, vec_b: &Vec<i64>) -> i64 {
    let mut similarity: i64 = 0;

    let mut freqs: HashMap<i64, i64> = HashMap::new();

    for b in vec_b.iter() {
        let count = freqs.entry(*b).or_insert(0);
        *count += 1;
    }

    for a in vec_a.iter() {
        let b_freq = freqs.get(&a).unwrap_or(&0);
        similarity += a * b_freq;
    }

    return similarity;
}

pub fn run() {
    println!("Day 01:");
    let lines = lines_from_file("./src/day01/input");
    // Part A
    let mut vec_a: Vec<i64> = Vec::new();
    let mut vec_b: Vec<i64> = Vec::new();

    for line in lines.iter() {
        vec_a.push(line[0]);
        vec_b.push(line[1]);
    }

    vec_a.sort();
    vec_b.sort();

    let sum = part_one(&vec_a, &vec_b);
    let similarity = part_two(&vec_a, &vec_b);

    println!("Part 1: {}", sum);
    println!("Part 2: {}", similarity);
}
