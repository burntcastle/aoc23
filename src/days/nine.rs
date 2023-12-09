use nom::IResult;

use crate::utils::{Input, ProblemInput};
use std::{collections::HashSet, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    9
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut result = 0;
    for line in lines {
        if let Ok((_, data)) = parse_line(line) {
            let diff = get_depth_answers(&data);
            let diff = *data.last().unwrap() + diff.iter().sum::<i64>();
            result += diff;
        }
    }
    result
}

fn get_depth_answers(input: &[i64]) -> Vec<i64> {
    let mut results: Vec<i64> = Vec::new();

    let mut input = input.to_owned();
    loop {
        let diffs = get_diffs(&input);
        input = diffs.clone();
        results.push(*diffs.last().unwrap());
        if diffs.into_iter().collect::<HashSet<i64>>().len() == 1 {
            // we have a result
            break;
        }
    }
    results
}

fn get_diffs(input: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for (i, item) in input.iter().skip(1).enumerate() {
        let diff = item - input[i];
        result.push(diff)
    }
    result
}

fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    let input = input.trim();
    let result = nom::multi::separated_list1(
        nom::character::complete::char(' '),
        nom::character::complete::i64,
    )(input)?;

    Ok(result)
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut result = 0;
    for line in lines {
        if let Ok((_, mut data)) = parse_line(line) {
            data.reverse();
            let diff = get_depth_answers(&data);
            let diff = *data.last().unwrap() + diff.iter().sum::<i64>();
            result += diff;
        }
    }
    result
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn one_parse_lines() {
        let input = "0 3 6 9 12 15";
        let input = parse_line(input).unwrap().1;
        assert_eq!(input, vec![0, 3, 6, 9, 12, 15])
    }

    #[test]
    fn one() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 114);
    }
    #[test]
    fn two() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
}
