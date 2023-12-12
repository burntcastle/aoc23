use itertools::Itertools;
use nom::number;

use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, iter::Enumerate, time::Instant};

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
impl Record {
    fn as_char(&self) -> char {
        match self {
            Record::Operational => '.',
            Record::Damaged => '#',
            Record::Unknown => '?',
            _ => panic!("Invalid Record"),
        }
    }
}

fn parse_input(line: &str) -> (Vec<Record>, Vec<i64>) {
    let text: Vec<&str> = line.split(" ").collect();
    let springs = text
        .first()
        .unwrap()
        .trim()
        .chars()
        .map(|x| Record::from(x))
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

fn is_valid(records: &Vec<Record>, nums: &Vec<i64>) -> bool {
    let mut record_slices: Vec<Vec<Record>> = Vec::new();
    let mut sub_slice: Vec<Record> = Vec::new();
    for record in records {
        if *record == Record::Operational {
            if sub_slice.len() > 0 {
                record_slices.push(sub_slice.clone());
                sub_slice.clear();
            }
        } else {
            sub_slice.push(record.clone());
        }
    }
    if sub_slice.len() > 0 {
        record_slices.push(sub_slice.clone());
    }
    if record_slices.len() != nums.len() {
        return false;
    }
    for (i, num) in nums.iter().enumerate() {
        match record_slices.get(i) {
            Some(slice) => {
                if slice.len() as i64 != *num {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    return true;

    // let mut current_len = 0;
    // let mut i = 0;
    // for record in records {
    //     let mut text = String::new();
    //     current_len += 1;
    //     if current_len > 1 && *record == Record::Damaged {
    //         if current_len != *nums.get(i).unwrap() {
    //             return false;
    //         }
    //         current_len = 0;
    //         i += 1;
    //     }
    // }
    // true
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut total = 0;
    for line in lines {
        let (springs, nums) = parse_input(line);
        
        let mut valid = 0;
        let unknowns = springs
            .clone()
            .into_iter()
            .filter(|x| *x == Record::Unknown)
            .count();
        let mut combos = vec![Record::Damaged; unknowns];
        combos.extend(vec![Record::Operational; unknowns]);
        let combos = vec![Record::Damaged, Record::Operational];
        //let count = combos.clone().iter().permutations(unknowns).unique().into_iter().collect::<Vec<Vec<&Record>>>().len();
        let it = itertools::repeat_n(combos, unknowns)
            .multi_cartesian_product()
            .unique();
        for mut combo in it {
            let mut input: Vec<Record> = vec![];
            for item in &springs {
                input.push(match item {
                    Record::Unknown => combo.pop().unwrap(),
                    _ => *item,
                })
            }
            if is_valid(&input, &nums) {
                //println!("Valid: {:?} {:?} {}", &input, nums,valid);

                valid += 1;
            }
        }

        //println!("{}:{}", line,valid);
        total += valid;
    }
    total
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut total = 0;

    for line in lines {
        let mut sub_total = 1;
        let (springs, nums) = parse_input(line);
        let mut springs_var: Vec<Vec<Record>> = vec![];
        let first = springs.first().unwrap();
        let last = springs.last().unwrap();
        let mut prefix =Record::Unknown;
        let mut suffix = Record::Unknown;
        let mut first_destroyed = false;
        let mut last_destroyed = false;

        if *last == Record::Damaged {
            last_destroyed = true;
            //prefix = Record::Operational;
        }
        if *first == Record::Damaged{
            first_destroyed = true;
            //suffix = Record::Operational;
        }

        for i in 0..5 {
            let mut sub_spring = springs.clone();

            if i < 4  { //&& (*springs.first().unwrap() != Record::Damaged &&  *springs.last().unwrap() != Record::Damaged) {
                //sub_spring.push(Record::Unknown);
                sub_spring.push(suffix);
            } 
            if i >0 {//
                //sub_spring.insert(0, Record::Unknown);
                sub_spring.insert(0, prefix);
            }

            // if i > 0 && i < 4 
            // { 
            //     sub_spring.push(match *springs.first().unwrap(){
            //         Record::Damaged => Record::Unknown,
            //         Record::Operational => Record::Unknown,
            //         Record::Unknown => Record::Unknown,
            //     });
                
            //     sub_spring.insert(0,match *springs.last().unwrap(){
            //         Record::Damaged => Record::Unknown,
            //         Record::Operational => Record::Unknown,
            //         Record::Unknown => Record::Unknown,
            //     });
            // }
            

            springs_var.push(sub_spring)
        }
        for (i,springs) in springs_var.into_iter().enumerate() {
            let mut valid = 0;
            let unknowns = springs
                .clone()
                .into_iter()
                .filter(|x| *x == Record::Unknown)
                .count();
            let mut combos = vec![Record::Damaged; unknowns];
            combos.extend(vec![Record::Operational; unknowns]);
            let combos = vec![Record::Damaged, Record::Operational];
            //let count = combos.clone().iter().permutations(unknowns).unique().into_iter().collect::<Vec<Vec<&Record>>>().len();
            let it = itertools::repeat_n(combos, unknowns)
                .multi_cartesian_product()
                .unique();
            for mut combo in it {
                let mut input: Vec<Record> = vec![];
                for item in &springs {
                    input.push(match item {
                        Record::Unknown => combo.pop().unwrap(),
                        _ => *item,
                    })
                }
                if is_valid(&input, &nums) {
                    
                    if i != 0 &&  (first_destroyed && *input.last().unwrap() == Record::Damaged  || last_destroyed && *input.first().unwrap() == Record::Damaged){
                        
                    }else {
                        
                        valid += 1;
                    }

                    
                }
            }
            sub_total *= valid;
        }
        //println!("Sub Total:{}", sub_total);
        total+= sub_total;
    }
    total
}
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 21;
    const PART_ONE_TEST: &str = "?????????????????????????????????????????????????????????????????????????? 7,2,7,2,7,2,7,2,7,2";
//         "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    const PART_TWO_ANSWER: i64 = 525152;
    const PART_TWO_TEST: &str = "?????????????? 7,2";
    // "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";

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
    fn fn_is_valid() {
        let input = "123";
        let (x, y) = parse_input(".###.##.#... 3,2,1");
        assert!(is_valid(&x, &y));

        let (x, y) = parse_input(".###.##....# 3,2,1");
        assert!(is_valid(&x, &y));

        let (x, y) = parse_input(".###.##.... 3,2,1");
        assert!(!is_valid(&x, &y));
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
        let input = ProblemInput::String(PART_TWO_TEST.trim());
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
