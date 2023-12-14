use memoize::memoize;

use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    12
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one_faster(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two_faster(input), now.elapsed())
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}
impl From<char> for Record {
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
    let text: Vec<&str> = line.split(' ').collect();
    let springs = text
        .first()
        .unwrap()
        .trim()
        .chars()
        .map(Record::from)
        .collect();

    let numbers: Vec<i64> = text
        .get(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x: &str| x.parse::<i64>().unwrap())
        .collect();
    (springs, numbers)
}

pub fn do_part_one_faster(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut total = 0;
    for line in lines {
        let (springs, nums) = parse_input(line);

        let result = count_valid(springs, nums);

        total += result;
    }
    total
}

pub fn do_part_two_faster(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut total = 0;
    for line in lines {
        let (springs, nums) = parse_input(line);
        let mut super_springs = springs.clone();
        let mut super_nums = nums.clone();
        for _i in 0..4 {
            super_springs.push(Record::Unknown);
            super_springs.append(&mut springs.clone());
            super_nums.append(&mut nums.clone());
        }
        let result = count_valid(super_springs, super_nums);
        //println!("{}:{}", line, result);
        total += result;
    }
    total
}

fn is_cluster_valid(num: i64, input: Vec<Record>) -> bool {
    if num > input.len() as i64 {
        return false;
    }

    for i in 0..num {
        if input[i as usize] == Record::Operational {
            return false;
        }
    }
    if num < input.len() as i64 && input[num as usize] == Record::Damaged {
        return false;
    }

    true
}

#[memoize]
fn count_valid(input: Vec<Record>, remaining_nums: Vec<i64>) -> i64 {
    let mut solutions = 0;

    //Check there is a next numer
    if remaining_nums.first().is_none() {
        if input.contains(&Record::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }
    let next_num = *remaining_nums.first().unwrap();

    if is_cluster_valid(next_num, input.clone()) {
        if input.len() > next_num as usize {
            solutions += count_valid(
                input[next_num as usize + 1..].to_vec(),
                remaining_nums[1..].to_vec(),
            );
        } else {
            solutions += count_valid(vec![], remaining_nums[1..].to_vec())
        }
    }

    if input.len() > next_num as usize
        && (input.first().unwrap() == &Record::Operational
            || input.first().unwrap() == &Record::Unknown)
    {
        solutions += count_valid(input[1..].to_vec(), remaining_nums.to_vec());
    }

    solutions
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 21;
    const PART_ONE_TEST: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    const PART_TWO_ANSWER: i64 = 525152;
    const PART_TWO_TEST: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
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
    fn other_faster() {
        let input = ProblemInput::String(".??..??...??#. 2,2");
        let result = do_part_one_faster(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn one_faster() {
        let input = ProblemInput::String(PART_ONE_TEST.trim());
        let result = do_part_one_faster(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST.trim());
        let result = do_part_two_faster(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
