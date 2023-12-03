use crate::utils::{Input, ProblemInput};
use std::{ io::BufRead, time::Instant};

pub fn the_day() -> u32 {
    4
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (u32, std::time::Duration) {
    todo!("Implement day {} part one",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (u32, std::time::Duration) {
    todo!("Implement day {} part two",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

pub fn do_part_one(input: Input) -> u32 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
0
}


fn do_part_two(input: Input) -> u32 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    0
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn test_part_one_single_line() {
        let input = "################";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 467 + 114);
    }
    #[test]
    fn test_part_one_multi_line() {
        let input = "################
################";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 4361);
    }
    fn test_part_two_single_line() {
        let input = "################";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 467835);
    }
    #[test]
    fn test_part_two_multi_line() {
        let input = "################
################";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 467835);
    }
}
