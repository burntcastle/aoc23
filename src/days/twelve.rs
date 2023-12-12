use nom::number;

use crate::utils::{Input, ProblemInput};
use std::{ io::BufRead, time::Instant, iter::Enumerate};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    12
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

#[derive(Debug, PartialEq)]
enum Record{
    Operational,
    Damaged,
    Unknown,
}
impl From<char> for Record{
    fn from(c: char) -> Self {
        match c {
            '.' => Record::Operational,
            '#' => Record::Damaged,
            '?' => Record::Unknown,
            _ => panic!("Invalid Record"),
        }
    }
}


fn parse_input(line: &str) -> (Vec<Record>, Vec<i64>) {
    let text: Vec<&str> = line.split(" ").collect();
    let springs = text.first().unwrap().trim().chars().map(|x| Record::from(x)).collect();
    let  numbers: Vec<i64> = text.get(1).unwrap().trim().split(',').map(|x| x.parse::<i64>().unwrap()).collect();
    (springs, numbers)
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    for line in lines{
        let (springs, nums) = parse_input(line);
        println!("{:?} {:?}", springs, nums);
    }
0
}

fn do_part_two(input: Input) -> i64 {
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

    
    const PART_ONE_ANSWER: i64 = 0;
    const PART_ONE_TEST: &str = "???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    const PART_TWO_ANSWER: i64 = 0;
    const PART_TWO_TEST:&str = "";

    #[test]
    fn panics() {
        let input = "Panic!";
        let res = std::panic::catch_unwind(|| parse_input("abc"));
        assert!(res.is_err());
        let res = std::panic::catch_unwind(|| parse_input("abc 1,2,3"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| parse_input("abc 1,b,3"));
        assert!(res.is_err());
    }

    #[test]
    fn fn_() {
        let input = "123";
        let result = input.parse::<i32>().unwrap();
        assert_eq!(result, 123);
    }


    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST.trim());
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }


    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 0);
    }

}
