use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Error},
    path::Path,
};

use crate::utils::Input;

pub fn part_one(input: Input) -> u32 {
    let lines = input.get_data().lines();

    let mut total = 0;
    for l in lines {
        let line = l.unwrap();
        let mut char_vec: Vec<char> = (&line).chars().collect();
        let mut first_number = 0;
        let mut number_location = 0;

        for c in &char_vec {
            match c.to_digit(10) {
                Some(x) => {
                    first_number = x;
                    break;
                }
                None => {
                    number_location += 1;
                }
            }
        }

        let mut second_number = 0;
        number_location = 0;
        char_vec.reverse();
        for c in &char_vec {
            match c.to_digit(10) {
                Some(x) => {
                    second_number = x;
                    break;
                }
                None => {
                    number_location += 1;
                }
            }
        }

        total += (first_number * 10) + second_number;
    }
    total
}

pub fn part_two(input: Input) -> u32 {
    let lines = input.get_data().lines();

    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let reverse_numbers: Vec<String> = numbers
        .clone()
        .iter()
        .map(|x| x.chars().rev().collect::<String>())
        .collect();
    let mut total = 0;
    for l in lines {
        let line = l.unwrap();
        let mut char_vec: Vec<char> = (&line).chars().collect();
        let mut first_number = 0;
        let mut number_location = 0;

        for c in &char_vec {
            match c.to_digit(10) {
                Some(x) => {
                    first_number = x;
                    break;
                }
                None => {
                    number_location += 1;
                }
            }
        }
        let mut i = 0;
        for num in &numbers {
            match line.find(num) {
                Some(x) => {
                    if x < number_location {
                        first_number = i;
                        number_location = x;
                    }
                }
                None => {}
            }
            i += 1;
        }

        let mut second_number = 0;
        number_location = 0;
        char_vec.reverse();
        for c in &char_vec {
            match c.to_digit(10) {
                Some(x) => {
                    second_number = x;
                    break;
                }
                None => {
                    number_location += 1;
                }
            }
        }
        let line: String = char_vec.iter().collect();
        let mut i = 0;
        for num in &reverse_numbers {
            let num = num.as_str();

            match line.find(num) {
                Some(x) => {
                    if x < number_location {
                        second_number = i;
                        number_location = x
                    }
                }
                None => {}
            }
            i += 1;
        }
        total += (first_number * 10) + second_number;
    }
    total
}



#[cfg(test)]
mod tests {
    use crate::utils::ProblemInput;

    use super::*;
    #[test]
    fn part_one_test_digits() {
        let input = "asdk1asfd2sdsdf4aa
        asdk1asfd2sdsdf4aa";
        let input = ProblemInput::String(input);
        let result = part_one(Input::new(input));
        assert_eq!(result,28);
    }
    
    #[test]
    fn part_two_test_digits() {
        let input = "asdk1asfd2sdsdf4aa
        asdk3asfd2sdsdf2aa";
        let input = ProblemInput::String(input);
        let result = part_two(Input::new(input));
        assert_eq!(result,46);
    }
    #[test]
    fn part_two_test_words() {
        let input = "athreesdk1asfd2sdsdf4sixzaa
        dfasfivelaksjdasthreeasdasoneasd";
        let input = ProblemInput::String(input);
        let result = part_two(Input::new(input));
        assert_eq!(result,87);
    }
    #[test]
    fn part_two_test_mixed() {
        let input = "asdninek1asfd2sdsdf4aa
        asdddzerosdfj4alsasfdl1lsdd";
        let input = ProblemInput::String(input);
        let result = part_two(Input::new(input));
        assert_eq!(result,95);
    }
}