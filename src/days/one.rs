use crate::utils::{Input, ProblemInput};
use std::{io::prelude::*, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let path = "./inputs/1";
    let input = ProblemInput::File(path);
    let input = Input::new(input);
    do_part_one(input)
}

fn do_part_one(input: Input) -> (i64, std::time::Duration) {
    let now = Instant::now();

    let lines = input.get_data().lines();

    let mut total = 0;
    for l in lines {
        let line = l.unwrap();
        let mut char_vec: Vec<char> = (line).chars().collect();
        let mut first_number = 0;
        //let mut number_location = 0;

        for c in &char_vec {
            if let Some(x) = c.to_digit(10) {
                first_number = x;
                break;
            }
        }

        let mut second_number = 0;
        //number_location = 0;
        char_vec.reverse();
        for c in &char_vec {
            if let Some(x) = c.to_digit(10) {
                second_number = x;
                break;
            }
        }

        total += (first_number * 10) + second_number;
    }
    (total as i64, now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let path = "./inputs/1";
    let input = ProblemInput::File(path);
    let input = Input::new(input);
    do_part_two(input)
}
pub fn do_part_two(input: Input) -> (i64, std::time::Duration) {
    let now = Instant::now();
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
        let mut char_vec: Vec<char> = (line).chars().collect();
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
        for (i, num) in numbers.iter().enumerate() {
            if let Some(x) = line.find(num) {
                if x < number_location {
                    first_number = i as u32;
                    number_location = x;
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
        let line: String = char_vec.iter().collect();
        for (i, num) in reverse_numbers.iter().enumerate() {
            let num = num.as_str();

            if let Some(x) = line.find(num) {
                if x < number_location {
                    second_number = i as u32;
                    number_location = x
                }
            }
        }
        total += (first_number * 10) + second_number;
    }
    (total as i64, now.elapsed())
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
        let result = do_part_one(Input::new(input));
        assert_eq!(result.0, 28);
    }

    #[test]
    fn part_two_test_digits() {
        let input = "asdk1asfd2sdsdf4aa
        asdk3asfd2sdsdf2aa";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        assert_eq!(result.0, 46);
    }
    #[test]
    fn part_two_test_words() {
        let input = "athreesdk1asfd2sdsdf4sixzaa
        dfasfivelaksjdasthreeasdasoneasd";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        assert_eq!(result.0, 87);
    }
    #[test]
    fn part_two_test_mixed() {
        let input = "asdninek1asfd2sdsdf4aa
        asdddzerosdfj4alsasfdl1lsdd";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        assert_eq!(result.0, 95);
    }
}
