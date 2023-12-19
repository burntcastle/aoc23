use std::{io, process::exit};
mod days;
mod utils;

#[cfg(not(tarpaulin_include))]
fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap_or(-1)
}

#[cfg(not(tarpaulin_include))]
fn main() {
    loop {
        println!("Enter day (0 to quit):");
        //let day = read_i32();
        let day = read_i32();
        let scores = match day {
            0 => {
                println!("Exiting...");
                exit(1);
            }
            1 => (Some(days::one::part_one()), Some(days::one::part_two())),
            2 => (Some(days::two::part_one()), Some(days::two::part_two())),
            3 => (Some(days::three::part_one()), Some(days::three::part_two())),
            4 => (Some(days::four::part_one()), Some(days::four::part_two())),
            5 => (Some(days::five::part_one()), Some(days::five::part_two())),
            6 => (Some(days::six::part_one()), Some(days::six::part_two())),
            7 => (Some(days::seven::part_one()), Some(days::seven::part_two())),
            8 => (Some(days::eight::part_one()), Some(days::eight::part_two())),
            9 => (Some(days::nine::part_one()), Some(days::nine::part_two())),
            10 => (Some(days::ten::part_one()), Some(days::ten::part_two())),
            11 => (
                Some(days::eleven::part_one()),
                Some(days::eleven::part_two()),
            ),
            12 => (
                Some(days::twelve::part_one()),
                Some(days::twelve::part_two()),
            ),
            13 => (
                Some(days::thirteen::part_one()),
                Some(days::thirteen::part_two()),
            ),
            14 => (
                Some(days::fourteen::part_one()),
                Some(days::fourteen::part_two()),
            ),
            15 => (
                Some(days::fifteen::part_one()),
                Some(days::fifteen::part_two()),
            ),
            16 => (
                Some(days::sixteen::part_one()),
                Some(days::sixteen::part_two()),
            ),
            17 => (
                Some(days::seventeen::part_one()),
                Some(days::seventeen::part_two()),
            ),
            18 => (
                Some(days::eighteen::part_one()),
                Some(days::eighteen::part_two()),
            ),
            19 => (
                Some(days::nineteen::part_one()),
                Some(days::nineteen::part_two()),
            ),
            _ => (None, None),
        };
        println!();
        match scores {
            (Some((r1, e1)), Some((r2, e2))) => {
                println!("Part One: {} ({:.2?})", r1, e1);
                println!("Part Two: {} ({:.2?})", r2, e2);
            }
            (Some((r1, e1)), None) => {
                println!("Part One: {} ({:.2?})", r1, e1);
                println!("Part Two Unimplemented");
            }
            (None, Some((r2, e2))) => {
                println!("Part One Unimplemented");
                println!("Part Two: {} ({:.2?})", r2, e2);
            }
            _ => {
                println!("Invalid Day")
            }
        };
        println!();
    }
}
