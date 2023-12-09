use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[derive(Clone, Copy)]
pub struct Bag {
    blue: u32,
    red: u32,
    green: u32,
}
impl Bag {
    fn count_from_color(&self, color: Colours) -> u32 {
        match color {
            Colours::Blue => self.blue,
            Colours::Red => self.red,
            Colours::Green => self.green,
        }
    }
    fn update_colour(&mut self, colour: Colours, count: u32) {
        match colour {
            Colours::Blue => self.blue = count,
            Colours::Red => self.red = count,
            Colours::Green => self.green = count,
        }
    }
}
#[derive(Copy, Clone)]
enum Colours {
    Blue,
    Red,
    Green,
}
impl Colours {
    fn from_string(colour: &str) -> Colours {
        match colour {
            "blue" => Colours::Blue,
            "red" => Colours::Red,
            "green" => Colours::Green,
            _ => panic!("Invalid colour"),
        }
    }
}
#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = "./inputs/2";
    let input = ProblemInput::File(path);
    let bag = Bag {
        blue: 14,
        red: 12,
        green: 13,
    };
    let input = Input::new(input);
    (do_part_one(input, bag), now.elapsed())
}
#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = "./inputs/2";
    let input = ProblemInput::File(path);
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

pub fn do_part_one(input: Input, bag: Bag) -> i64 {
    let lines = input.get_data().lines();
    //get game number
    let mut i = 1;
    let mut valid_games = 0;
    for line in lines {
        let mut is_valid = true;
        let line = line.unwrap();
        let split = line.split(':');
        let mut split = split.collect::<Vec<&str>>();
        split[1] = split[1].trim();
        let results = split[1].split(';');

        //results now represents the colors
        for result in results {
            let result = result.trim();
            let colour_vals = result.split(',');
            for colour_val in colour_vals {
                let colour_val = colour_val.trim();
                let colour_val = colour_val.split(' ');
                let colour_val = colour_val.collect::<Vec<&str>>();
                let colour = Colours::from_string(colour_val[1]);
                let colour_count = colour_val[0].parse::<u32>().unwrap();
                if colour_count > bag.count_from_color(colour) {
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid {
            valid_games += i;
        }
        i += 1;
    }

    valid_games as i64
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    //get game number
    let mut power = 0;
    for line in lines {
        let mut min_bag = Bag {
            blue: 0,
            red: 0,
            green: 0,
        };
        let line = line.unwrap();
        let split = line.split(':');
        let mut split = split.collect::<Vec<&str>>();
        split[1] = split[1].trim();
        let results = split[1].split(';');

        //results now represents the colors
        for result in results {
            let result = result.trim();
            let colour_vals = result.split(',');
            for colour_val in colour_vals {
                let colour_val = colour_val.trim();
                let colour_val = colour_val.split(' ');
                let colour_val = colour_val.collect::<Vec<&str>>();
                let colour = Colours::from_string(colour_val[1]);
                let colour_count = colour_val[0].parse::<u32>().unwrap();
                if colour_count > min_bag.count_from_color(colour) {
                    min_bag.update_colour(colour, colour_count);
                }
            }
        }
        power += min_bag.blue * min_bag.red * min_bag.green;
    }

    power as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    #[should_panic]
    fn invalid_colour() {
        let _colour = Colours::from_string("purple");
    }

    #[test]
    fn one_single_line() {
        let bag = Bag {
            blue: 14,
            red: 12,
            green: 13,
        };

        let input = "Game 1: 30 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input), bag);
        println!("Result: {}", result);
        assert_eq!(result, 0);

        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input), bag);
        println!("Result: {}", result);
        assert_eq!(result, 1);
    }
    #[test]
    fn one_multi_line() {
        let bag = Bag {
            blue: 14,
            red: 12,
            green: 13,
        };

        let input = "Game 1: 30 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 3 blue, 40 red; 1 red, 2 green, 6 blue; 2 green
        Game 3: 3 blue, 4 red; 1 red, 24 green, 6 blue; 2 green
        Game 4: 3 blue, 4 red; 1 red, 1 green, 6 blue; 2 green
        Game 5: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input), bag);
        println!("Result: {}", result);
        assert_eq!(result, 9);
    }

    #[test]
    fn two_single_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        // esult = 6*4*2 = 48
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 48);
    }
    #[test]
    fn two_multi_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 9 blue, 4 red; 1 red, 2 green, 6 blue; 3 green";
        // result = 6*4*2 + 9*4*3 = 156
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 156);
    }
}
