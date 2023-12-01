use std::io;

use crate::utils::{Input, ProblemInput};

mod days;
mod utils;

fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap_or(-1)
}

fn main() {
    loop {
        println!("Enter day (0 to quit):");
        let day = read_i32();
        match day {
            0 => {
                println!("Exiting...");
                break;
            }
            1 => {
                let path = "./inputs/1/one";
                let input = ProblemInput::File(path);
                let result = days::one::part_one(Input::new(input));
                println!("Part One: {}", result);
                let result = days::one::part_two(Input::new(input));
                println!("Part Two: {}", result);
            }
            _ => {
                println!("Invalid Day")
            }
        }
        println!();
    }
}
