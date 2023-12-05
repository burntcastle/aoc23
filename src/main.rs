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
            1 => {              
                (Some(days::one::part_one()),Some(days::one::part_two()))
            }
            2 => {
                (Some(days::two::part_one()),Some(days::two::part_two()))
            }
            3 =>{
                (Some(days::three::part_one()),Some(days::three::part_two()))
            }
            4 =>{
                (Some(days::four::part_one()),Some(days::four::part_two()))
            }
            5 =>{
                (Some(days::five::part_one()),None)
            }
            _ => {
                (None,None)
            }
        };

        match scores {
            (Some((r1,e1)),Some((r2,e2))) => {
                println!("Part One: {} ({:.2?})", r1, e1);
                println!("Part Two: {} ({:.2?})", r2, e2);
            }
            (Some((r1,e1)),None) => {
                println!("Part One: {} ({:.2?})", r1, e1);
                println!("Part Two Unimplemented");
            }            
            (None,Some((r2,e2))) => {
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
