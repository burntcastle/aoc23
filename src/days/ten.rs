use crate::utils::{Input, ProblemInput};
use std::{ io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    10
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    todo!("Implement day {} part one",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    todo!("Implement day {} part two",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}",the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}



pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
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
    const PART_ONE_TEST: &str = "";

    const PART_TWO_ANSWER: i64 = 0;
    const PART_TWO_TEST:&str = "";

    #[test]
    fn panics() {
        let input = "Panic!";
        let res = std::panic::catch_unwind(|| panic!("{}",input));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| part_one());
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| part_two());
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
        assert_eq!(result, 0);
    }

}
