use crate::utils::{Input, ProblemInput};
use core::panic;

use nom::{self, IResult};

use std::collections::HashMap;
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    8
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

fn decode(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, key) = nom::bytes::complete::take_while(|c| c != ' ')(input)?;
    let (input, _) = nom::bytes::complete::tag(" = (")(input)?;
    let (input, left) = nom::bytes::complete::take_until(", ")(input)?;
    let (input, _) = nom::bytes::complete::tag(", ")(input)?;
    let (input, right) = nom::bytes::complete::take_until(")")(input)?;
    Ok((input, (key, left, right)))
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let directions = lines.first().unwrap().chars().collect::<Vec<char>>();
    let mut instructions: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }

        let (_, (key, left, right)) = decode(line.trim()).unwrap();

        if instructions.insert(key, (left, right)).is_some() {
            panic!("Duplicate key found: {}", key)
        }
    }

    let mut i: i64 = 0;
    let mut finished = false;
    let mut next = "AAA";
    while !finished {
        let index = i as usize % (directions.len());
        let direction = directions.get(index).unwrap();
        let (left, right) = instructions.get(next).unwrap();
        next = get_next(direction, left, right);
        if next == "ZZZ" {
            finished = true;
        }
        i += 1;
    }
    i
}

fn get_next<'a>(direction: &'a char, left: &'a str, right: &'a str) -> &'a str {
    match direction {
        'L' => left,
        'R' => right,
        _ => panic!("Invalid direction: {}", direction),
    }
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let directions = lines.first().unwrap().chars().collect::<Vec<char>>();
    let mut instructions: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }
        let (_, (key, left, right)) = decode(line.trim()).unwrap();

        if instructions.insert(key, (left, right)).is_some() {
            panic!("Duplicate key found: {}", key)
        }
    }

    let next: Vec<&str> = instructions
        .keys()
        .filter(|&x| x.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    let mut first: HashMap<&str, usize> = HashMap::new();
    let mut repeats: HashMap<&str, usize> = HashMap::new();

    for start in next {
        let mut finished = false;
        let mut next_item = start;
        let mut found_first = false;
        let mut found_progress_along_directions = 0;
        let mut found_it = 0;
        let mut i: usize = 0;
        while !finished {
            // how far along
            let progress_along_directions = i % directions.len();

            let direction = directions.get(progress_along_directions).unwrap();

            let (l, r) = instructions.get(&next_item).unwrap();
            next_item = get_next(direction, l, r);

            if next_item.ends_with('Z') && !found_first {
                first.insert(start, i + 1);
                found_first = true;
                found_it = i + 1;
                found_progress_along_directions = progress_along_directions + 1;
            } else if next_item.ends_with('Z')
                && found_first
                && (progress_along_directions + 1) == found_progress_along_directions
            {
                repeats.insert(start, i - found_it);
                finished = true;
            }
            i += 1;
        }
    }

    let mut total = *first.values().next().unwrap();
    for &val in first.values().skip(1) {
        total = num::integer::lcm(total, val);
        //total = lcm(total, val);
    }

    total as i64
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;
    #[test]
    #[should_panic]
    fn one_panic() {
        let input = "RL

        AAA = (BBB, CCC)
        AAA = (BBB, CCC)";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
    #[test]
    #[should_panic]
    fn one_panic_two() {
        let input = "XL

        AAA = (BBB, CCC)";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
    #[test]
    fn one_simple() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
    #[test]
    fn one_repeat() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 6);
    }
    #[test]
    fn two() {
        let input = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 6);
    }
    #[test]
    #[should_panic]
    fn two_panic() {
        let input = "RL

        AAA = (BBB, CCC)
        AAA = (BBB, CCC)";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
}
