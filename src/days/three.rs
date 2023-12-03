use crate::utils::{Input, ProblemInput};
use std::{collections::HashMap, io::BufRead, time::Instant};

pub fn part_one() -> (u32, std::time::Duration) {
    let now = Instant::now();
    let path = "./inputs/3";
    let input = ProblemInput::File(path);
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

pub fn part_two() -> (u32, std::time::Duration) {
    let now = Instant::now();
    let path = "./inputs/3";
    let input = ProblemInput::File(path);
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

pub fn symbols() -> Vec<char> {
    vec!['\n', '#', '$', '%', '&', '*', '+', '-', '/', '=', '@']
}
pub fn is_symbol(c: char) -> bool {
    symbols().contains(&c)
}

pub fn do_part_one(input: Input) -> u32 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let mut total = 0;
    let mut above: Option<&str> = None;
    let mut below: Option<&str>;

    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        if i > 0 {
            above = lines.get(i - 1).copied();
        }
        below = lines.get(i + 1).copied();

        let splits = line.split(|c: char| c == '.' || is_symbol(c));
        let mut place_along_string = 0;

        for split in splits {
            let split = split.trim();
            match split.parse::<u32>() {
                Ok(x) => {
                    if check_row(
                        above,
                        below,
                        line,
                        place_along_string,
                        place_along_string + (split.len() as i32),
                    ) {
                        total += x;
                    }

                    place_along_string += split.len() as i32 + 1;
                }
                Err(_) => place_along_string += 1,
            }
        }
    }

    total
}
fn check_row(
    above: Option<&str>,
    below: Option<&str>,
    current: &str,
    start: i32,
    end: i32,
) -> bool {
    let length = current.len();
    // the || is a lazy or so only executes if the first is false
    let results = (match above {
        Some(x) => {
            let x: Vec<char> = x.chars().collect();
            check_for_char(x.get(get_slice_size(start, end, length)))
        }
        None => false,
    } || match below {
        Some(x) => {
            let x: Vec<char> = x.chars().collect();
            check_for_char(x.get(get_slice_size(start, end, length)))
        }
        None => false,
    }) || {
        let x: Vec<char> = current.chars().collect();
        check_for_char(x.get(get_slice_size(start, end, length)))
    };

    results
}
fn get_slice_size(start: i32, end: i32, length: usize) -> std::ops::Range<usize> {
    let start = start - 1;
    let end = end + 1;
    let mut out_start = 0;
    let mut out_end = 0;
    match start {
        x if x < 0 =>(),
        x if x > length as i32 => out_start = length,
        _ => out_start = start as usize,
    }
    match end {
        x if x < 0 => (),
        x if x > length as i32 => out_end = length,
        _ => out_end = end as usize,
    }
    out_start..out_end
}

fn check_for_char(char: Option<&[char]>) -> bool {
    match char {
        Some(x) => x.iter().any(|c| is_symbol(*c)),
        _ => false,
    }
}

fn do_part_two(input: Input) -> u32 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let mut numbers = vec![vec![(0, 0); lines.first().unwrap().len()]; lines.len()];
    let mut num_unique = 0;
    // Build vector of numbers
    for (i,line) in lines.iter().enumerate() {
        let splits = line.split(|c: char| c == '.' || is_symbol(c));
        let mut place_along_string = 0;
        for split in splits {
            let split = split.trim();
            match split.parse::<u32>() {
                Ok(x) => {
                    num_unique += 1;
                    for y in 0..split.len() {
                        numbers[i][y + place_along_string] = (num_unique, x);
                    }
                    place_along_string += split.len() + 1;
                }
                Err(_) => place_along_string += 1,
            }
        }
    }
    let mut total = 0;
    for (i,line) in lines.iter().enumerate() {
        let chars = line.chars();
        for (j, c) in chars.enumerate() {
            if c == '*' {
                total += get_gear_ratio(i, j, &numbers);
            }
        }
    }

    total
}

fn get_gear_ratio(row: usize, col: usize, numbers: &[Vec<(u32, u32)>]) -> u32 {
    //let nums: Vec<(u32, u32)> = vec![];
    //let numbers = numbers.clone();
    let mut results: HashMap<u32, u32> = HashMap::new();
    for i in row - 1..row + 2 {
        for j in col - 1..col + 2 {
            let row = numbers.get(i);

            if let Some(x) = row {
                let col = x.get(j);
                if let Some(x) = col {
                    let (k, v) = x;
                    results.insert(*k, *v);
                }
            }
        }
    }

    results.remove_entry(&0);
    if results.len() == 2 {
        let y: Vec<&u32> = results.values().clone().collect();
        return y[0] * y[1];
    }
    0
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn test_part_one_single_line() {
        let input = "*........#
467...114.
.........";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 467 + 114);
    }
    #[test]
    fn test_part_one_multi_line() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_two_multi_line() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 467835);
    }
}
