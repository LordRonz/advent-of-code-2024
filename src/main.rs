use std::env;

#[path = "day01/solution.rs"]
mod day01;

#[path = "day02/solution.rs"]
mod day02;

#[path = "day03/solution.rs"]
mod day03;

#[path = "day04/solution.rs"]
mod day04;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].parse::<u32>() {
            Ok(day) => match day {
                1 => day01::run(),
                2 => day02::run(),
                3 => day03::run(),
                4 => day04::run(),
                _ => println!("Unknown day: {}. Available days are 1, 2, 3, 4.", day),
            },
            Err(_) => println!("Invalid input. Please provide a number for the day."),
        }
    } else {
        // No argument provided, run all
        println!("No specific day provided. Running all solutions...");
        day01::run();
        day02::run();
        day03::run();
        day04::run();
    }
}
