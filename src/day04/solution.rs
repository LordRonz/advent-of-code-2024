use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn get_matrix_char_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let f = BufReader::new(File::open(filename).unwrap());

    f.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn search(matrix: &Vec<Vec<char>>, i: i64, j: i64) -> i64 {
    let search_x = [0, 0, 1, 1, 1, -1, -1, -1];
    let search_y = [-1, 1, -1, 0, 1, -1, 0, 1];

    let target = ['X', 'M', 'A', 'S'];

    let mut found = 0;

    for k in 0..8 {
        for l in 0..target.len() {
            let x = i + search_x[k] * l as i64;
            let y = j + search_y[k] * l as i64;
            if x < 0 || x >= matrix.len() as i64 || y < 0 || y >= matrix[0].len() as i64 {
                break;
            }
            if matrix[x as usize][y as usize] != target[l] {
                break;
            }
            if l == target.len() - 1 {
                found += 1;
            }
        }
    }

    found
}

fn search_two(matrix: &Vec<Vec<char>>, i: i64, j: i64) -> i64 {
    if matrix[i as usize][j as usize] != 'A' {
        return 0;
    }

    let top_left = matrix[i as usize - 1][j as usize - 1];
    let top_right = matrix[i as usize - 1][j as usize + 1];
    let bottom_left = matrix[i as usize + 1][j as usize - 1];
    let bottom_right = matrix[i as usize + 1][j as usize + 1];

    let diagonal_1 =
        top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M';
    let diagonal_2 =
        top_right == 'M' && bottom_left == 'S' || top_right == 'S' && bottom_left == 'M';

    return (diagonal_1 && diagonal_2) as i64;
}

fn part_one(matrix: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    for row in 0..matrix.len() {
        for cell in 0..matrix[row].len() {
            count += search(matrix, row as i64, cell as i64);
        }
    }
    count
}

fn part_two(matrix: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    for row in 1..matrix.len() - 1 {
        for cell in 1..matrix[row].len() - 1 {
            count += search_two(matrix, row as i64, cell as i64);
        }
    }
    count
}

pub fn run() {
    println!("Day 04:");
    let matrix = get_matrix_char_from_file("./src/day04/input");
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}
