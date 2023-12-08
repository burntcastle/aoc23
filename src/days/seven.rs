use crate::utils::{Input, ProblemInput};
use core::panic;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, io::BufRead, time::Instant};

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
        let line = line.trim().split(' ').collect::<Vec<&str>>();
        let hand = line[0].trim();
        let bid = line[1].trim().parse::<i64>().unwrap();
        results.push((hand, bid));
    }
    results.sort_by(|a, b| sort_hands(a.0, b.0));
    let mut total = 0;
    for (i, (_k, v)) in results.iter().enumerate() {
        total += (i as i64 + 1) * v;
    }
    total
}

fn sort_hands(hand_a: &str, hand_b: &str) -> Ordering {
    let ht_a = get_hand_type(hand_a);
    let ht_b = get_hand_type(hand_b);

    match &ht_a.cmp(&ht_b) {
        Ordering::Equal => {
            let hand_a = hand_a.chars().collect::<Vec<char>>();
            let hand_b = hand_b.chars().collect::<Vec<char>>();
            for i in 0..5 {
                let a_value = get_value_of_card(&hand_a[i]);
                let b_value = get_value_of_card(&hand_b[i]);
                match &a_value.cmp(&b_value) {
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                }
            }
            panic!("Shouldn't have identical hands")
        }
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
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
    let max_count = result.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
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
        _ => {
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
        let line = line.trim().split(' ').collect::<Vec<&str>>();
        let hand = line[0].trim();
        let bid = line[1].trim().parse::<i64>().unwrap();
        results.push((hand, bid));
    }
    results.sort_by(|a, b| sort_hands_two(a.0, b.0));

    let mut total = 0;
    for (i, (_k, v)) in results.iter().enumerate() {
        total += (i as i64 + 1) * v;
    }
    total
}

fn sort_hands_two(hand_a: &str, hand_b: &str) -> Ordering {
    let ht_a = get_hand_type_two(hand_a);
    let ht_b = get_hand_type_two(hand_b);
    match &ht_a.cmp(&ht_b) {
        Ordering::Equal => {
            let hand_a = hand_a.chars().collect::<Vec<char>>();
            let hand_b = hand_b.chars().collect::<Vec<char>>();
            for i in 0..5 {
                let a_value = get_value_of_card_two(&hand_a[i]);
                let b_value = get_value_of_card_two(&hand_b[i]);
                match &a_value.cmp(&b_value) {
                    Ordering::Greater => {
                        return Ordering::Greater;
                    }
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => {}
                }
            }
            panic!("No items in loop");
        }
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
    }
}

fn get_value_of_card_two(card: &char) -> i64 {
    let cards: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    13 - cards.iter().position(|&x| x == *card).unwrap() as i64
}

fn get_hand_type_two(hand: &str) -> i64 {
    if hand == "JJJJJ" {
        return 0;
    }
    let cards = hand.chars().collect::<Vec<char>>();
    if cards.contains(&'J') {
        let other_cards = cards.iter().filter(|&x| *x != 'J').collect::<Vec<&char>>();
        let other_cards: Vec<char> = other_cards.into_iter().copied().collect();
        let other_cards_str = other_cards.iter().cloned().collect::<String>();
        let other_cards_str = other_cards_str.as_str();
        //println!("  {}", other_cards_str);
        let mut other_cards_full: Vec<char> = vec![];
        for _i in 0..(5 - other_cards.len()) {
            other_cards_full.extend(other_cards.clone());
        }
        let combos = other_cards_full.iter().combinations(5 - other_cards.len());

        let mut min_score = 10;
        let mut _best = "".to_string();
        for combo in combos {
            let combo = combo.iter().cloned().collect::<String>();
            let combo = combo.as_str();
            let hand_str = format!("{}{}", other_cards_str, combo);
            let score = get_hand_type_three(&hand_str);
            if score < min_score {
                min_score = score;
                _best = hand_str.clone();
            }
        }

        min_score
    } else {
        get_hand_type_three(hand)
    }
}

fn get_hand_type_three(hand: &str) -> i64 {
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
    let max_count = result.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
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
        _ => {
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
    fn test_panics() {
        let res = std::panic::catch_unwind(|| sort_hands("KK234", "KK234"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type("XXXXX123"));
        assert!(res.is_err());
        let res = std::panic::catch_unwind(|| get_hand_type_two("XXXXX123"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type_three("XXXXX123"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_value_of_card(&'X'));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_value_of_card_two(&'X'));
        assert!(res.is_err());

        let input = "33333 1
        2233 3";
        let input = ProblemInput::String(input);
        let res = std::panic::catch_unwind(|| do_part_one(Input::new(input)));
        assert!(res.is_err());

        let input = "33333 1
        234 3";
        let input = ProblemInput::String(input);
        let res = std::panic::catch_unwind(|| do_part_one(Input::new(input)));
        assert!(res.is_err());

        let input = "2345678 1
        234 3";
        let input = ProblemInput::String(input);
        let res = std::panic::catch_unwind(|| do_part_one(Input::new(input)));
        assert!(res.is_err());

        let input = "33333 1
        2233 3";
        let input = ProblemInput::String(input);
        let res = std::panic::catch_unwind(|| sort_hands_two("23456", "23456"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type_three("KKQQ"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type_three("5KQ"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type_three("23675KQ"));
        assert!(res.is_err());

        let res = std::panic::catch_unwind(|| get_hand_type("23675KQ"));
        assert!(res.is_err());
    }
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

        let input = "33333 1
        22223 2
        22233 3";
        let input = ProblemInput::String(input);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 10);
        assert_eq!(Ordering::Less, sort_hands("KQ234", "KQ236"));
    }

    #[test]
    fn test_part_two_multi_line() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 5905);

        let input = "JJJJJ";
        assert_eq!(get_hand_type_two(input), 0);

        let input = "KKKKJ";
        assert_eq!(get_hand_type_two(input), 0);

        let input = "JJJJJ";
        assert_eq!(get_hand_type_two(input), 0);

        let input = "KKKKJ";
        assert_eq!(get_hand_type_two(input), 0);

        let input = "QQQQQ";
        assert_eq!(get_hand_type_three(input), 0);

        let input = "JJJJJ";
        assert_eq!(get_hand_type_three(input), 0);

        let input = "KKKKQ";
        assert_eq!(get_hand_type_three(input), 1);

        let input = "KKKQQ";
        assert_eq!(get_hand_type_three(input), 2);

        let input = "KKKKJ";
        assert_eq!(get_hand_type_three(input), 1);

        let input = "KKKQ8";
        assert_eq!(get_hand_type_three(input), 3);
    }

    #[test]
    fn test_part_two_multi_line_two() {
        let input = "JAAKK 1
        JJJAK 2";
        let input = ProblemInput::String(input);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 5);
    }
}
