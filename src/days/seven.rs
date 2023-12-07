use crate::utils::{Input, ProblemInput};
use core::panic;
use std::{collections::HashMap, io::BufRead, time::Instant, cmp::Ordering};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    7
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
    todo!("Implement day {} part two", the_day());
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
    let mut results: Vec<(&str, i64)> = vec![];
    for line in lines {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let hand = line[0].trim();
        let bid = line[1].trim().parse::<i64>().unwrap();
        results.push((hand, bid));
    }
    results.sort_by(|a, b| sort_hands(a.0, b.0));
    let mut total = 0;
    for (i, (k,v)) in results.iter().enumerate() {
        total += (i as i64 + 1) * v;
    }   
    total
}

fn sort_hands(hand_a: &str, hand_b: &str) -> Ordering {
    
    let ht_a = get_hand_type(hand_a);
    let ht_b = get_hand_type(hand_b);
    
    if ht_a == ht_b {
        let hand_a = hand_a.chars().collect::<Vec<char>>();
        let hand_b = hand_b.chars().collect::<Vec<char>>();
        for i in 0..5 {
            let a_value = get_value_of_card(&hand_a[i]);
            let b_value = get_value_of_card(&hand_b[i]);
            if a_value > b_value {
                //println!("{} > {}", a_value, b_value);
                return Ordering::Greater;
            } else if a_value < b_value {
                //println!("{} < {}", a_value, b_value);
                return Ordering::Less;
            }
        }
        panic!("Shouldn't have identical hands")

    } else if ht_a < ht_b {
        return Ordering::Greater;
    } else if ht_b < ht_a{
        return Ordering::Less;
    }else{
    panic!("Shouldn't have identical hands or get herre?")
    }
}

fn get_value_of_card(card: &char) -> i64 {
    let cards = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    12 - cards.iter().position(|&x| x == *card).unwrap() as i64
}

fn get_hand_type(hand: &str) -> i64 {
    let cards = hand.chars().collect::<Vec<char>>();

    // Five of a kind 0
    // Four of a kind 1
    // Full house 2
    // Three of a kind 3
    // Two pair 4
    // One pair 5
    // High card 6

    let mut result: HashMap<i64, i64> = HashMap::new();
    for card in cards {
        let card = get_value_of_card(&card);
        match result.contains_key(&card) {
            true => {
                result.insert(card, result.get(&card).unwrap() + 1);
            }
            false => {
                result.insert(card, 1);
            }
        }
    }
    let mut result: Vec<(&i64, &i64)> = result.iter().collect();
  
    result.sort_by(|a, b| b.1.cmp(a.1));
    let max_count = result.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    // now calculate the result
    match result.len() {
        1 => {
            // Five of a kind
            0
        }
        2 => {
            // Four of a kind
            match *max_count.1 {
                4 => 1,
                3 => 2,
                _ => panic!("Invalid hand"),
            }
        }
        3 => {
            match *max_count.1 {
                3 => 3, // three of a kind
                2 => 4, // two pair
                _ => panic!("Invalid hand"),
            }
        }
        4 => {
            // One pair
            5
        }
        5 => {
            // High card
            6
        }
        _ =>{
            panic!("Invalid hand")
        }
    }
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut results: Vec<(&str, i64)> = vec![];
    for line in lines {
        let line = line.trim().split(" ").collect::<Vec<&str>>();
        let hand = line[0].trim();
        let bid = line[1].trim().parse::<i64>().unwrap();
        results.push((hand, bid));
    }
    results.sort_by(|a, b| sort_hands(a.0, b.0));
    let mut total = 0;
    for (i, (k,v)) in results.iter().enumerate() {
        total += (i as i64 + 1) * v;
    }   
    total
}

fn sort_hands_two(hand_a: &str, hand_b: &str) -> Ordering {
    
    let ht_a = get_hand_type_two(hand_a);
    let ht_b = get_hand_type_two(hand_b);
    
    if ht_a == ht_b {
        let hand_a = hand_a.chars().collect::<Vec<char>>();
        let hand_b = hand_b.chars().collect::<Vec<char>>();
        for i in 0..5 {
            let a_value = get_value_of_card_two(&hand_a[i]);
            let b_value = get_value_of_card_two(&hand_b[i]);
            if a_value > b_value {
                //println!("{} > {}", a_value, b_value);
                return Ordering::Greater;
            } else if a_value < b_value {
                //println!("{} < {}", a_value, b_value);
                return Ordering::Less;
            }
        }
        panic!("Shouldn't have identical hands")

    } else if ht_a < ht_b {
        return Ordering::Greater;
    } else if ht_b < ht_a{
        return Ordering::Less;
    }else{
    panic!("Shouldn't have identical hands or get herre?")
    }
}


fn get_value_of_card_two(card: &char) -> i64 {
    let cards = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    ];

    12 - cards.iter().position(|&x| x == *card).unwrap() as i64
}

fn get_hand_type_two(hand: &str) -> i64 {
    let cards = hand.chars().collect::<Vec<char>>();

    // Five of a kind 0
    // Four of a kind 1
    // Full house 2
    // Three of a kind 3
    // Two pair 4
    // One pair 5
    // High card 6

    let mut result: HashMap<i64, i64> = HashMap::new();
    for card in cards {
        let card = get_value_of_card_two(&card);
        match result.contains_key(&card) {
            true => {
                result.insert(card, result.get(&card).unwrap() + 1);
            }
            false => {
                result.insert(card, 1);
            }
        }
    }
    let mut result: Vec<(&i64, &i64)> = result.iter().collect();
  
    result.sort_by(|a, b| b.1.cmp(a.1));
    let max_count = result.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    // now calculate the result
    match result.len() {
        1 => {
            // Five of a kind
            0
        }
        2 => {
            // Four of a kind
            match *max_count.1 {
                4 => 1,
                3 => 2,
                _ => panic!("Invalid hand"),
            }
        }
        3 => {
            match *max_count.1 {
                3 => 3, // three of a kind
                2 => 4, // two pair
                _ => panic!("Invalid hand"),
            }
        }
        4 => {
            // One pair
            5
        }
        5 => {
            // High card
            6
        }
        _ =>{
            panic!("Invalid hand")
        }
    }
}



#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;


    #[test]
    fn test_part_one_multi_line() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 6440);
    }

//     #[test]
//     fn test_part_two_multi_line() {
//         let input = "################
// ################";
//         let input = ProblemInput::String(input);
//         let result = do_part_two(Input::new(input));
//         println!("Result: {}", result);
//         assert_eq!(result, 467835);
//     }
}
