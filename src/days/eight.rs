use crate::utils::{Input, ProblemInput};
use core::panic;
use nom::*;
use nom::{self, branch, sequence::delimited, IResult};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::string;
use regex_macro::regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::{io::BufRead, ops::Range, time::Instant};

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

// fn parse_range(input: &str) -> IResult<&str,  RangeInclusive<u32>>{
//     let (input, start) = nom::character::complete::u32(input)?;
//     let (input, _) = nom::bytes::complete::tag("-")(input)?;
//     let (input, end) = nom::character::complete::u32(input)?;

//     Ok((input, start..=end))
// }

// fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)>{

//     let (input,(start,end)) = nom::sequence::separated_pair(parse_range, nom::bytes::complete::tag(","), parse_range)(input)?;
//     Ok((input, (start,end)))
// }

// fn do_sections(input: &str) ->  IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>>{

//     let (_, ranges) = nom::multi::separated_list1(nom::character::complete::line_ending, parse_line)(input)?;
//     Ok(("", ranges))
// }

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
    for (i, line) in lines.iter().skip(1).enumerate() {
        if line.is_empty() {
            continue;
        }

        let (_, (key, left, right)) = decode(line.trim()).unwrap();
        match instructions.insert(key, (left, right)) {
            Some(x) => {
                panic!("Duplicate key found: {}", key);
            }
            None => {}
        }
    }

    let mut i: i64 = 0;
    let mut finished = false;
    let mut next = "AAA";
    while !finished {
        let index = i as usize % (directions.len());
        let direction = directions.get(index).unwrap();

        //println!("Direction: {}, Next: {}", direction, next );

        let (left, right) = instructions.get(next).unwrap();
        next = get_next(direction, left, right);
        //println!("Direction: {}, Next: {}", direction, next );
        if next == "ZZZ" {
            finished = true;
        }
        i = i + 1
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
    for (i, line) in lines.iter().skip(1).enumerate() {
        if line.is_empty() {
            continue;
        }
        let (_, (key, left, right)) = decode(line.trim()).unwrap();
        match instructions.insert(key, (left, right)) {
            Some(x) => {
                panic!("Duplicate key found: {}", key);
            }
            None => {}
        }
    }


    let mut next: Vec<&str> = instructions
        .keys()
        .filter(|&x| x.ends_with('A'))
        .map(|x| *x)
        .collect::<Vec<&str>>();

    // println!("Start: {:?}", next);
    // while !finished {
    //     //println!("Next: {:?}", next);
    //     let index = i as usize % (directions.len());
    //     let direction = directions.get(index).unwrap();

    //     let mut next_locations: Vec<&str> = vec![];
    //     for n in next{
    //         let (l,r) = instructions.get(n).unwrap();
    //         next_locations.push(get_next(direction, l, r))
    //     }
    //     next = next_locations.clone();
    //     finished = is_finished(next.clone());
    //     i = i + 1
    // }
    let mut first: HashMap<&str, usize> = HashMap::new();
    let mut repeats: HashMap<&str, usize> = HashMap::new();
    next.iter().for_each(|start|{
        let mut finished = false;
        let mut next_item = *start;
        println!("Start: {:?}", next_item);
        let mut found_first = false;
        let mut found_index = 0;
        let mut i: i64 = 0;
        while !finished {

            //println!("Next: {:?}", next);
            let index = i as usize % (directions.len());
            let direction = directions.get(index).unwrap();

            let mut next_locations: Vec<&str> = vec![];
            
            let (l, r) = instructions.get(next_item).unwrap();
            next_item = (get_next(direction, l, r));
            
            i = i + 1;
            if(next_item.ends_with('Z') && !found_first){
                found_first = true;
                found_index = index;
                println!("Start: {}, ended at: {}, after: {}", start, next_item, i);
                first.insert(start, i as usize);
            }else if next_item.ends_with('Z') && index == found_index as usize{
                println!("Start: {}, repeated_after: {}", start,  i - found_index as i64);
                repeats.insert(start, i as usize);
                finished = true;
            }
            //i = i + 1
        }
    }); 
    println!("Firsts: {}",first.values().sum::<usize>());
    println!("Seconds: {}",repeats.values().sum::<usize>());
    
    for i in RangeInclusive::new(10, 1000000){

        //let mut results: Option<usize> = None;
        for (j,f) in first.values().enumerate(){
            let s  = repeats.values().nth(j).unwrap();
            if f > &i{
                break;
            }
            let mut z  =i-f;
            if z % s != 0{
                break;
            }
            z = z/s;
            }
            println!("Got to end with i:{}", i);
            return i as i64;
    }
    
    0
}

fn is_finished(next: Vec<&str>) -> bool {
    for n in next {
        if !n.ends_with('Z') {
            return false;
        }
    }
    true
}
fn get_next_part_two<'a>(direction: &'a char, locations: Vec<(&'a str, &'a str)>) -> Vec<&'a str> {
    let mut next: Vec<&str> = vec![];
    for (l, r) in locations {
        match direction {
            'L' => next.push(l),
            'R' => next.push(r),
            _ => panic!("Invalid direction: {}", direction),
        }
    }
    next
}
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn test_part_one_simple() {
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
    fn test_part_one_repeat() {
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
    fn test_part_two() {
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
}
