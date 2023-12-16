use crate::utils::{Input, ProblemInput};
use core::panic;
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    15
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
    let mut total = 0;
    let lines = lines.first().unwrap();
    for step in lines.split(',') {
        total += hash(step.trim());
    }
    total
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Operation {
    Remove,
    Add,
}

struct Lens {
    box_name: String,
    focal_length: i64,
}
impl Lens {
    fn from_string(input: &str) -> (Lens, Operation) {
        if input.contains('=') {
            let input = input.split('=').collect::<Vec<&str>>();
            let x = Lens {
                box_name: input[0].to_string(),
                focal_length: input[1].parse::<i64>().unwrap(),
            };
            (x, Operation::Add)
        } else if input.contains('-') {
            let input = input.split('-').collect::<Vec<&str>>();
            let x = Lens {
                box_name: input[0].to_string(),
                focal_length: 0,
            };
            (x, Operation::Remove)
        } else {
            panic!("Invalid Action");
        }
    }
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let lines = lines.first().unwrap();
    let mut boxes: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut focal_lengths: HashMap<i64, i64> = HashMap::new();
    let mut keys: Vec<String> = Vec::new();
    for step in lines.split(',') {
        let (lens, op) = Lens::from_string(step);
        let box_no = hash(lens.box_name.as_str());
        let box_name = lens.box_name.clone();
        let box_id;
        if keys.contains(&box_name) {
            box_id = keys.iter().position(|x| *x == box_name).unwrap() as i64;
        } else {
            box_id = keys.len() as i64;
            keys.push(box_name);
        }
        if op == Operation::Add {
            let focal_length = lens.focal_length;
            let box_list = boxes.entry(box_no).or_insert(Vec::new());
            if !box_list.contains(&box_id) {
                box_list.push(box_id);
            }
            focal_lengths.insert(box_id, focal_length);
        } else if op == Operation::Remove {
            let box_list = boxes.entry(box_no).or_insert(Vec::new());
            let idx = &box_list.iter().position(|x| *x == box_id);
            match idx {
                Some(idx) => {
                    box_list.remove(*idx);
                }
                None => {
                    // Not there!
                }
            }
        } else {
        }
        //println!("{:?}", boxes);
    }
    let mut total = 0;
    for box_no in boxes.keys() {
        let x = box_no + 1;
        for (i, lens) in boxes.get(box_no).unwrap().iter().enumerate() {
            total += (i as i64 + 1) * x * focal_lengths.get(lens).unwrap();
        }
    }
    total
}

fn hash(step: &str) -> i64 {
    let chars = step.chars();
    let mut i = 0;
    for c in chars {
        i = hash_step(i, to_asci_code(c));
    }
    i
}

fn to_asci_code(c: char) -> i64 {
    c as i64
}

fn hash_step(i: i64, code: i64) -> i64 {
    let mut i = i + code;
    i *= 17;
    i %= 256;
    i
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 1320;
    const PART_ONE_TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    const PART_TWO_ANSWER: i64 = 145;
    const PART_TWO_TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn panics() {
        let input = "Panic!";

        // let res = std::panic::catch_unwind(|| Operation::new('a'));
        // assert!(res.is_err());

        let res = std::panic::catch_unwind(|| Lens::from_string("askdja"));
        assert!(res.is_err());
    }

    #[test]
    fn fn_() {
        assert_eq!(to_asci_code('='), 61);
        assert_eq!(to_asci_code('a'), 97);
        assert_eq!(to_asci_code('r'), 114);
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
        assert_eq!(result, 145);
    }
}
