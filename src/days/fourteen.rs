use crate::utils::{Input, ProblemInput};
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    14
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

fn get_rows_and_cols(line_groups: &[&str]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rows: Vec<Vec<i32>> = Vec::new();
    let mut cols: Vec<Vec<i32>> = Vec::new();
    // each puzzle

    for line in line_groups {
        let mut row = vec![0; line.len()];
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                row[i] = -1;
            } else if c == 'O' {
                row[i] = 1;
            }
        }

        rows.push(row);
    }
    for (i, _c) in rows[0].iter().enumerate() {
        let mut col = vec![];
        for row in &rows {
            col.push(row[i]);
        }
        cols.push(col);
    }
    (rows, cols)
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (rows, cols) = get_rows_and_cols(&lines);
    let mut total = 0;
    for col in cols {
        let load = rows.len();
        //
        let mut start = 0;
        for (_i, subset) in col.split(|num| *num == -1).enumerate() {
            let rocks = subset.iter().filter(|&x| *x == 1).collect::<Vec<&i32>>();
            for (i, _rock) in rocks.iter().enumerate() {
                total += load - (i + start);
            }
            start += 1 + subset.len();
        }
    }
    total as i64
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

fn do_tilt(data: Vec<Vec<i32>>, dir: Dir) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    for row in data {
        out.push(do_row_tilt(row, dir))
    }
    out
}

fn do_row_tilt(mut col: Vec<i32>, dir: Dir) -> Vec<i32> {
    let mut out_col: Vec<i32> = Vec::new();
    let mut first = true;
    if dir == Dir::Right {
        col.reverse();
    }
    for subset in col.split(|num| *num == -1) {
        if !first {
            out_col.push(-1);
        }
        let rocks_count = subset.iter().filter(|&x| *x == 1).count();
        let rocks = &mut vec![1; rocks_count];
        let spaces = &mut vec![0; subset.len() - rocks_count];
        out_col.append(rocks);
        out_col.append(spaces);
        first = false;
    }
    if dir == Dir::Right {
        out_col.reverse();
    }
    assert!(col.len() == out_col.len());
    out_col
}

fn transpose(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    for i in 0..input.first().unwrap().len() {
        let mut inner: Vec<i32> = Vec::new();
        for input_j in &input {
            inner.push(input_j[i]);
        }
        out.push(inner);
    }
    out
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let (_rows, cols) = get_rows_and_cols(&lines);
    let mut data = cols.clone();
    let mut store: HashMap<Vec<Vec<i32>>, i32> = HashMap::new();
    let mut rev_store: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    let (mut loop_start, mut loop_finish) = (0, 0);

    for i in 0..100000000 {
        data = do_tilt(data, Dir::Left);
        data = transpose(data);

        // tilt west
        data = do_tilt(data, Dir::Left);
        data = transpose(data);

        // tilt south
        data = do_tilt(data, Dir::Right);
        data = transpose(data);

        //tilt east
        data = do_tilt(data, Dir::Right);
        data = transpose(data);

        if store.contains_key(&data) {
            loop_start = *store.get(&data).unwrap();
            loop_finish = i + 1;
            break;
        } else {
            store.insert(data.clone(), i + 1);
            rev_store.insert(i + 1, data.clone());
        }
    }

    let cycle = loop_finish - loop_start;
    let modulo = loop_start + (1000000000 - loop_start) % cycle;
    let data = rev_store.get(&modulo).unwrap();
    let mut total = 0;
    for col in data {
        for (i, row) in col.iter().enumerate() {
            if *row == 1 {
                total += col.len() - i;
            }
        }
    }

    total as i64
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 136;
    const PART_ONE_TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const PART_TWO_ANSWER: i64 = 64;
    //  b

    const PART_TWO_TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn fn_transpose() {
        let input = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 0]];
        let result = transpose(input.clone());
        let answer = vec![vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 0]];
        println!("{:?}", input);
        println!("{:?}", result);
        println!("{:?}", answer);
        assert_eq!(result, answer);
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
