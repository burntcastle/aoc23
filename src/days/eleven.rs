use crate::utils::{Input, ProblemInput};
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    11
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
    let mut points: HashMap<usize, (usize, usize)> = HashMap::new();

    let mut data: Vec<Vec<i64>> = Vec::new();
    let mut i = 1;
    for line in lines {
        let mut row: Vec<i64> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '#' => {
                    row.push(i);
                    i += 1
                }
                x => println!("Invalid input: {}", x),
            }
        }
        data.push(row);
    }
    let length = &data[0].clone().len();
    let mut new_data: Vec<Vec<i64>> = Vec::new();
    for row in data.clone() {
        new_data.push(row.clone());
        if row.iter().sum::<i64>() == 0 {
            new_data.push(row.clone());
        }
    }
    let mut cols: Vec<usize> = Vec::new();
    for i in 0..*length {
        let col: Vec<i64> = data.iter().map(|x| x[i]).collect::<Vec<i64>>().clone();
        if col.iter().sum::<i64>() == 0 {
            cols.push(i);
        }
    }
    let mut final_data = new_data.clone();
    for (i, _) in new_data.iter().enumerate() {
        let mut offset = 0;
        for col in cols.clone() {
            final_data.get_mut(i).unwrap().insert(col + offset, 0);
            offset += 1;
        }
    }

    for (i, row) in final_data.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col != &0 {
                points.insert(*col as usize, (i, j));
            }
        }
    }
    let mut total_length = 0;
    for i in 0..points.len() {
        let i = i + 1;
        let start = points.get(&i).unwrap();
        for j in i..points.len() {
            let j = j + 1;
            let diff = start.0.abs_diff(points.get(&j).unwrap().0)
            + start.1.abs_diff(points.get(&j).unwrap().1);
            
            println!("{}-{}:{}", i,j, diff);
            total_length += diff;
        }
    }

    total_length.try_into().unwrap()
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut points: HashMap<usize, (usize, usize)> = HashMap::new();

    let mut data: Vec<Vec<i64>> = Vec::new();
    let mut i = 1;
    for line in lines {
        let mut row: Vec<i64> = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '#' => {
                    row.push(i);
                    i += 1
                }
                x => println!("Invalid input: {}", x),
            }
        }
        data.push(row);
    }
    for row in data.clone() {
       for col in row{
        print!("{}", col   );

       }
       println!();
    }
    
    let length = &data[0].clone().len();
    
    let mut rows: Vec<usize> = Vec::new();
    for (i, row) in data.iter().enumerate() {
        if row.iter().sum::<i64>() == 0 {
            rows.push(i);
        }
    }
    let mut cols: Vec<usize> = Vec::new();
    for i in 0..*length {
        let col: Vec<i64> = data.iter().map(|x| x[i]).collect::<Vec<i64>>().clone();
        if col.iter().sum::<i64>() == 0 {
            cols.push(i);
        }
    }

    for (i, row) in data.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col != &0 {
                points.insert(*col as usize, (i, j));
            }
        }
    }
    println!("Rows: {:?}", rows);
    println!("Cols: {:?}", cols);
    println!("Length: {}", points.len());
    let mut total_length = 0;
    for i in 0..points.len() {
        let i = i + 1;
        let start = points.get(&i).unwrap();
        for j in i..points.len() {
            let j = j +1;
            let (x,y) = points.get(&j).unwrap();

            let x_dif = start.0.abs_diff(*x);
            let y_dif = start.1.abs_diff(*y);

            let x_m = rows.iter().filter(|&z|z> &start.0 && z < x).count();
            let y_m = cols.iter().filter(|&z|z> &start.1 && z < y).count();
            let additional = (y_m + x_m) *(2-1);
            total_length += x_dif + y_dif + additional;
            println!("{}-{}:{}", i,j, x_dif + y_dif + additional);
        }
    }
    println!("Total Length: {}", total_length);
    total_length.try_into().unwrap()
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 371;
    const PART_ONE_TEST: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    const PART_TWO_ANSWER: i64 = 0;
    const PART_TWO_TEST: &str = "...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";

    // #[test]
    // fn panics() {
    //     let input = "Panic!";
    //     let res = std::panic::catch_unwind(|| panic!("{}", input));
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
        assert_eq!(result, 0);
    }
}
