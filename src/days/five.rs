#![allow(unused)]
use crate::utils::{Input, ProblemInput};
use kdam::tqdm;
use rayon::prelude::*;
use std::{
    io::BufRead,
    ops::{ControlFlow, Range},
    time::Instant,
    vec,
};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    5
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
    let seeds = lines.first().unwrap().split(':').collect::<Vec<&str>>();
    let seeds = seeds[1].trim().split(' ').collect::<Vec<&str>>();

    let mut seeds = seeds
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut rows: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut row: Vec<(i64, i64, i64)> = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut conversions: Vec<(i64, i64, i64)> = vec![];
        // skip line 1
        if i < 2 || line.trim().contains("-to-") {
            continue;
        } else if line.trim().is_empty() {
            rows.push(row);
            row = vec![];
            continue;
        } else {
            let conversion = line.trim().split(' ').collect::<Vec<&str>>();
            let conversion = conversion
                .iter()
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let dest = conversion[0];
            let source = conversion[1];
            let len = conversion[2];
            let item = (dest, source, len);
            row.push(item);
        }
    }

    for row in rows.iter() {
        seeds.par_iter_mut().for_each(|seed| {
            let result = row
                .iter()
                .filter(|(d, s, l)| *seed >= *s && *seed < (*s + *l))
                .collect::<Vec<&(i64, i64, i64)>>()
                .first()
                .copied();
            if let Some((d, s, l)) = result {
                *seed += (d - s);
            }
        })
    }

    //println!("FINAL:{:?}", seeds);
    *seeds.iter().min().unwrap()
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let seeds = lines.first().unwrap().split(':').collect::<Vec<&str>>();
    let seeds = seeds[1].trim().split(' ').collect::<Vec<&str>>();

    let mut seeds = seeds
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut complete_seeds: Vec<i64> = vec![];

    //for (i, seed) in tqdm!(seeds.iter().enumerate(), desc="Calculating complete seeds", position=0) {
    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 1 {
            complete_seeds.extend((seeds[i - 1]..(seeds[i - 1] + *seed)).collect::<Vec<i64>>());
        }
    }
    let mut seeds = complete_seeds;
    let mut rows: Vec<Vec<(i64, i64, i64)>> = vec![];
    let mut row: Vec<(i64, i64, i64)> = vec![];
    //for (i, line) in tqdm!(lines.iter().enumerate(), desc = "Parsing lines", position = 1)
    for (i, line) in lines.iter().enumerate() {
        let mut conversions: Vec<(i64, i64, i64)> = vec![];
        // skip line 1
        if i < 2 || line.trim().contains("-to-") {
            continue;
        } else if line.trim().is_empty() {
            rows.push(row);
            row = vec![];
            continue;
        } else {
            let conversion = line.trim().split(' ').collect::<Vec<&str>>();
            let conversion = conversion
                .iter()
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let dest = conversion[0];
            let source = conversion[1];
            let len = conversion[2];
            let item = (dest, source, len);
            row.push(item);
        }
    }

    //for row in tqdm!(rows.iter(), desc = "Calculating locations", position = 2)
    for row in rows.iter() {
        seeds.par_iter_mut().for_each(|seed| {
            let result = row
                .iter()
                .filter(|(d, s, l)| *seed >= *s && *seed < (*s + *l))
                .collect::<Vec<&(i64, i64, i64)>>()
                .first()
                .copied();
            if let Some((d, s, l)) = result {
                *seed += (d - s);
            }
        })
    }

    //println!("FINAL:{:?}", seeds);
    *seeds.iter().min().unwrap()
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn one_multi_line() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 35);
    }

    #[test]
    fn two_multi_line() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4";

        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 46);
    }
}
