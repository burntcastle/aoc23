use rayon::range;

use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    13
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
    for line_groups in lines.split(|x| x.is_empty()) {
        let (rows, cols) = get_rows_and_cols(line_groups);

        let row_sym = find_symmetry(&rows);
        let col_sym = find_symmetry(&cols);
        total += row_sym * 100 + col_sym;
    }

    total.into()
}

fn find_symmetry(rows: &Vec<Vec<i32>>) -> i32 {
    'search: for i in 1..rows.len() {
        let num_above = rows.len() - i;
        let num_below = i;
        let steps = num_above.min(num_below);
        for j in 0..steps {
            let below = &rows[i - j - 1];
            let above = &rows[i + j];
            if below != above {
                continue 'search;
            }
        }
        return i as i32;
    }
    0
}

fn find_symmetry_with_one_diff(rows: &Vec<Vec<i32>>) -> i32 {
    'search: for i in 1..rows.len() {
        let mut differences = 0;
        let num_above = rows.len() - i;
        let num_below = i;
        let steps = num_above.min(num_below);
        for j in 0..steps {
            let below = &rows[i - j - 1];
            let above = &rows[i + j];
            for (i, b) in below.iter().enumerate() {
                if b != &above[i] {
                    differences += 1;
                    if differences > 1 {
                        continue 'search;
                    }
                }
            }
        }
        if differences == 1 {
            return i as i32;
        }
    }
    0
}

fn get_rows_and_cols(line_groups: &[&str]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let mut cols: Vec<Vec<i32>> = Vec::new();
    // each puzzle

    for line in line_groups {
        let mut row = vec![0; line.len()];
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                row[i] = 1;
            }
        }

        rows.push(row);
    }
    for (i, _c) in rows[0].iter().enumerate() {
        let mut col = vec![0; rows.len()];
        for row in &rows {
            col.push(row[i]);
        }
        cols.push(col);
    }
    (rows, cols)
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut total = 0;
    for line_groups in lines.split(|x| x.is_empty()) {
        let (rows, cols) = get_rows_and_cols(line_groups);

        let row_sym = find_symmetry_with_one_diff(&rows);
        let col_sym = find_symmetry_with_one_diff(&cols);
        total += row_sym * 100 + col_sym;
    }

    total.into()
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 405;
    const PART_ONE_TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const PART_TWO_ANSWER: i64 = 400;
    const PART_TWO_TEST: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    // #[test]
    // fn panics() {
    //     let input = "Panic!";
    //     let res = std::panic::catch_unwind(|| panic!("{}",input));
    //     assert!(res.is_err());

    //     let res = std::panic::catch_unwind(|| part_one());
    //     assert!(res.is_err());

    //     let res = std::panic::catch_unwind(|| part_two());
    //     assert!(res.is_err());
    // }

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
