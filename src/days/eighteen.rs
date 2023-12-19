use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    18
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part one",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part two",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

fn parse_input(input: Vec<&str>) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    let mut current_loc = (0, 0);
    //res.push(current_loc.clone());
    for row in input {
        let mut row = row.split(' ');
        let instr = row.next().unwrap();
        let val = row.next().unwrap().parse::<i64>().unwrap();
        match instr {
            "R" => {
                current_loc.0 += val;
            }
            "L" => {
                current_loc.0 -= val;
            }
            "D" => {
                current_loc.1 += val;
            }
            "U" => {
                current_loc.1 -= val;
            }
            unknown => {
                panic!("Error: {:?}", unknown);
            }
        }
        res.push(current_loc);
    }
    res
}

fn parse_input_updated(input: Vec<&str>) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    let mut current_loc = (0, 0);
    //res.push(current_loc.clone());
    for row in input {
        let mut row = row.split(' ');
        let _ = row.next().unwrap();
        let _ = row.next().unwrap().parse::<i64>().unwrap();
        let data = row.next().unwrap().trim();

        let val = i64::from_str_radix(&data[2..data.len() - 2], 16).unwrap();

        let instr = data.chars().collect::<Vec<char>>();
        let instr = instr[instr.len() - 2];
        match instr {
            '0' => {
                current_loc.0 += val;
            }
            '2' => {
                current_loc.0 -= val;
            }
            '1' => {
                current_loc.1 += val;
            }
            '3' => {
                current_loc.1 -= val;
            }
            unknown => {
                panic!("Error: {:?}", unknown);
            }
        }
        res.push(current_loc);
    }
    res
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let parsed = parse_input(lines);

    // let width = parsed.clone().iter().map(|x| x.0).max().unwrap() as usize + 1;
    // let heigth = parsed.clone().iter().map(|x| x.1).max().unwrap() as usize + 1;
    let mut area = 0;

    let mut perimeter = 0;
    for (i, (x, y)) in parsed.iter().enumerate() {
        let (x_prev, y_prev) = parsed[(i + 1) % (parsed.len())];
        area += x_prev * y - x * y_prev;
        perimeter += (x - x_prev).abs() + (y - y_prev).abs();
    }

    let interior = area.abs() / 2 - perimeter / 2 + 1;
    interior + perimeter
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let parsed = parse_input_updated(lines);
    // let width = parsed.clone().iter().map(|x| x.0).max().unwrap() as usize + 1;
    // let heigth = parsed.clone().iter().map(|x| x.1).max().unwrap() as usize + 1;
    let mut area = 0;
    // let mut print = vec![vec!['.'; width]; heigth];

    // let mut y_gcd = 1;
    // let mut x_gcd = 1;
    // for (x,y) in parsed.iter() {
    //     print!("({}, {}), ", x, y);
    //     if *x != 0{
    //         x_gcd = num::integer::gcd(x_gcd, *x);
    //     }
    //     if *y != 0 {
    //         y_gcd = num::integer::gcd(y_gcd, *y);
    //     }
    //     println!("GCD: {}, {}", x_gcd, y_gcd);
    // }
    // println!("GCD: {}, {}", x_gcd, y_gcd);
    let mut perimeter = 0;
    for (i, (x, y)) in parsed.iter().enumerate() {
        let (x_prev, y_prev) = parsed[(i + 1) % (parsed.len())];
        area += x_prev * y - x * y_prev;
        perimeter += (x - x_prev).abs() + (y - y_prev).abs();
    }
    let interior = area.abs() / 2 - perimeter / 2 + 1;
    interior + perimeter
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 62;
    const PART_ONE_TEST: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    const PART_TWO_ANSWER: i64 = 952408144115;
    const PART_TWO_TEST: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    #[should_panic]
    fn panics_one() {
        let input = ProblemInput::String("XD 5 (#0dc57X)");
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
    }
    #[test]
    #[should_panic]
    fn panics_two() {
        let input = ProblemInput::String("XD 5 (#0dc57X)");
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
    }

    #[test]
    fn fn_() {
        let input = "123";
        let result = input.parse::<i32>().unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
