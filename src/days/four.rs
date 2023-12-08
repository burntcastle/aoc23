use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    4
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
    for line in lines {
        total += get_card_score(line);
    }
    total as i64
}

fn get_card_score(card: &str) -> u32 {
    let splits = card.split(|c: char| c == ':' || c == '|');
    let splits = splits.collect::<Vec<&str>>();
    let left = splits[1].trim();
    let right = splits[2].trim();
    let left = left.replace("  ", " 0");
    let right = right.replace("  ", " 0");
    let left: Vec<u32> = left.split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
    let right: Vec<u32> = right
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    get_overlap_score(left, right)
}

fn get_overlap_score(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut overlap = 0;
    for item in left {
        if right.contains(&item) {
            match overlap {
                0 => overlap = 1,
                _ => overlap *= 2,
            }
        }
    }
    overlap
}

fn get_card_count(card: &str) -> u32 {
    let splits = card.split(|c: char| c == ':' || c == '|');
    let splits = splits.collect::<Vec<&str>>();
    let left = splits[1].trim();
    let right = splits[2].trim();
    let left = left.replace("  ", " 0");
    let right = right.replace("  ", " 0");
    let left: Vec<u32> = left.split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
    let right: Vec<u32> = right
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    get_overlap_count(left, right)
}

fn get_overlap_count(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut overlap = 0;
    for item in left {
        if right.contains(&item) {
            overlap += 1
        }
    }
    overlap
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut cards: Vec<&str> = lines.clone();
    for (i, _line) in lines.iter().enumerate() {
        cards.extend(get_winning_cards(lines.clone(), i, 0));
    }
    //     for line in cards{
    //         total+= get_card_score(line);
    //     }
    cards.len() as i64
}

fn get_winning_cards(cards: Vec<&str>, current: usize, step: usize) -> Vec<&str> {
    let step = step + 1;
    // helpful for debugging
    // let indent =  "-".repeat(step);
    // println!("{}{}",indent, current+1);
    let mut results: Vec<&str> = vec![];
    let score = get_card_count(cards[current]) as usize;
    for i in (current + 1)..(current + score + 1) {
        results.push(cards[i]);
        for sub_result in get_winning_cards(cards.clone(), i, step) {
            results.push(sub_result);
        }
    }
    results
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    #[test]
    fn test_part_one_single_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_one_single_line_two() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part_one_multi_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 13);
    }
    // fn test_part_two_single_line() {
    //     let input = "################";
    //     let input = ProblemInput::String(input);
    //     let result = do_part_two(Input::new(input));
    //     println!("Result: {}", result);
    //     assert_eq!(result, 467835);
    // }
    #[test]
    fn test_part_two_multi_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 30);
    }
}
