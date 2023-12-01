use std::io;

mod days;

fn read_i32() -> i32 {
    
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap_or(-1)
}

fn main() {
    loop{
    println!("Enter day (0 to quit):");
    let day = read_i32();
    match day {
        0 => {
            println!("Exiting...");
            break;
        },
        1=>{
            println!("Day {}",day);
            let result = days::one::part_one();
            println!("Part One: {}",result);
            let result = days::one::part_two();
            println!("Part Two: {}",result);
        },
        _=>{
            println!("Invalid Day")
        }
    }
    println!();
}

    


}
