use crate::utils::{Input, ProblemInput};
use std::{ io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    6
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

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut times: Vec<i64> = vec![];
    let mut distances: Vec<i64> = vec![];
    for line in lines {
        if line.contains("Time"){
            let time = line.split(':').collect::<Vec<&str>>();
            let time = time[1].trim().split(' ').collect::<Vec<&str>>();
            for t in time {
                if t.trim().is_empty() {
                    continue;
                }
                times.push(t.trim().parse::<i64>().unwrap());
            }
        } else if line.contains("Distance"){
            let distance = line.split(':').collect::<Vec<&str>>();
            let distance = distance[1].trim().split(' ').collect::<Vec<&str>>();
            for d in distance {
                if d.trim().is_empty() {
                    continue;
                }
                distances.push(d.trim().parse::<i64>().unwrap());
            }
        }
    }

    let mut total: i64 = 1;    
    for (i,time) in times.iter().enumerate() {
        let distance = distances[i];
        let mut possible :Vec<i64>=vec![];
        for h in 0..time+1{
            let travelled =   h*time -(h.pow(2));
            if travelled > distance{
                possible.push(h);
            }
        }
        let possible = possible.into_iter()
        .collect::<std::collections::HashSet<i64>>()
        .into_iter()
        .collect::<Vec<i64>>().len();
        total *= possible as i64;
    }
    total
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut time: i64 = 0;
    let mut distance: i64 = 0;
    for line in lines {
        if line.contains("Time"){
            let line = line.split(':').collect::<Vec<&str>>();
            time = line[1].replace(' ', "").parse::<i64>().unwrap();
            
        } else if line.contains("Distance"){
            let line = line.split(':').collect::<Vec<&str>>();
            distance = line[1].replace(' ', "").parse::<i64>().unwrap();
        }
    }
    let minima = (time as f64 - ((time as f64).powf(2.0) - 4.0 * distance as f64).sqrt())/ 2.0;
    let maxima = (time as f64 + ((time as f64).powf(2.0) - 4.0 * distance as f64).sqrt())/2.0;
   maxima.floor() as i64 - minima.ceil() as i64 +1
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn test_part_one_multi_line() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result,288);
    }

    #[test]
    fn test_part_two_single_line() {
        let input = "################";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_multi_line() {
        let input = "Time:      7  15   30
        Distance:  9  40  200";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 71503);
    }
}
