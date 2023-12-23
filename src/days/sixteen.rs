use crate::utils::{Input, ProblemInput};
use std::{collections::HashSet, io::BufRead, time::Instant};
#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    16
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part one",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part two",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Tile {
    Empty,
    ReflectBack,
    ReflectForward,
    SplitHorizontal,
    SplitVertical,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    p: Vec<(Point, Direction)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}
impl Board {
    fn new(tiles: Vec<Vec<Tile>>) -> Board {
        let width = tiles[0].len();
        let height = tiles.len();
        Board {
            tiles,
            width,
            height,
        }
    }
    fn get_tile(&self, point: Point) -> Option<Tile> {
        let (x, y) = (point.x, point.y);
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.tiles[y][x])
        }
    }
    fn is_valid(&self, point: Point) -> bool {
        let (x, y) = (point.x, point.y);
        x < self.width && y < self.height
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

fn get_next_directions(tile: Tile, direction: Direction) -> Vec<Direction> {
    let mut results: Vec<Direction> = Vec::new();
    match tile {
        Tile::Empty => {
            results.push(direction);
        }
        Tile::ReflectBack => match direction {
            Direction::Right => {
                results.push(Direction::Down);
            }
            Direction::Down => {
                results.push(Direction::Right);
            }
            Direction::Left => {
                results.push(Direction::Up);
            }
            Direction::Up => {
                results.push(Direction::Left);
            }
        },
        Tile::ReflectForward => match direction {
            Direction::Right => {
                results.push(Direction::Up);
            }
            Direction::Down => {
                results.push(Direction::Left);
            }
            Direction::Left => {
                results.push(Direction::Down);
            }
            Direction::Up => {
                results.push(Direction::Right);
            }
        },
        Tile::SplitHorizontal => match direction {
            Direction::Right | Direction::Left => {
                results.push(direction);
            }
            Direction::Down | Direction::Up => {
                results.push(Direction::Right);
                results.push(Direction::Left);
            }
        },
        Tile::SplitVertical => match direction {
            Direction::Down | Direction::Up => {
                results.push(direction);
            }
            Direction::Right | Direction::Left => {
                //this order matters for the first cell
                results.push(Direction::Down);
                results.push(Direction::Up);
            }
        },
    }

    results
}

fn create_step(
    path: &[(Point, Direction)],
    next_direction: Direction,
    board: &Board,
) -> Option<(Point, Direction)> {
    let (mut next_point, _last_direction) = *path.last().unwrap();

    match next_direction {
        Direction::Right => {
            next_point.x += 1;
        }
        Direction::Down => {
            next_point.y += 1;
        }
        Direction::Left => {
            if next_point.x == 0 {
                return None;
            }
            next_point.x -= 1;
        }
        Direction::Up => {
            if next_point.y == 0 {
                return None;
            }
            next_point.y -= 1;
        }
    }
    // check if step is valid
    if board.is_valid(next_point) {
        // need to check for loops, the 'state' is location and next direction
        if path.contains(&(next_point, next_direction)) {
            //println!("Board contains step");
            None
        } else {
            //path.push();
            Some((next_point, next_direction))
        }
    } else {
        //println!("step invalid");
        None
    }
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::ReflectBack,
            '/' => Tile::ReflectForward,
            '-' => Tile::SplitHorizontal,
            '|' => Tile::SplitVertical,
            _ => panic!("Invalid tile char: {}", c),
        }
    }
    // fn to_char(tile: Tile) -> char {
    //     match tile {
    //         Tile::Empty => '.',
    //         Tile::ReflectBack => '\\',
    //         Tile::ReflectForward => '/',
    //         Tile::SplitHorizontal => '-',
    //         Tile::SplitVertical => '|',
    //     }
    // }
}

fn build_board(input: Vec<&str>) -> Board {
    let mut results: Vec<Vec<Tile>> = Vec::new();
    for line in input {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c));
        }
        results.push(row);
    }
    Board::new(results)
}

// fn print_board(board: &Board, results: &HashSet<Point>) {
//     for y in 0..board.height {
//         for x in 0..board.width {
//             match results.get(&Point::new(x, y)) {
//                 Some(_) => print!("#"),
//                 None => print!(
//                     "{}",
//                     Tile::to_char(board.get_tile(Point::new(x, y)).unwrap())
//                 ),
//             }
//         }
//         println!();
//     }
//     println!();
// }

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let board = build_board(lines);
    let start = Point::new(0, 0);

    //relying on the order here!
    let mut path = vec![(start, Direction::Right)];
    recursive_wrapper(&mut path, &board);
    let results: HashSet<Point> = path.iter().map(|x| x.0).collect();
    //print_board(&board, &results);
    results.len() as i64
}

fn recursive_wrapper(path: &mut Vec<(Point, Direction)>, board: &Board) {
    let (next_point, direction) = *path.last().unwrap();

    let next_directions = get_next_directions(board.get_tile(next_point).unwrap(), direction);

    for next_direction in next_directions {
        //print_board(board, &points_touched);
        path.push((next_point, direction));
        let new_point = create_step(path, next_direction, board);
        //println!("new_point: {:?}", new_point);
        if let Some(new_point) = new_point {
            path.push(new_point);
            recursive_wrapper(path, board);
        }
    }
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let board = build_board(lines);

    let mut results: Vec<usize> = Vec::new();
    for y in 0..board.height {
        //relying on the order here!
        let start = Point::new(0, y);
        let mut path = vec![(start, Direction::Right)];
        recursive_wrapper(&mut path, &board);
        results.push(path.iter().map(|x| x.0).collect::<HashSet<Point>>().len());
        let start = Point::new(board.width - 1, y);
        let mut path = vec![(start, Direction::Left)];
        recursive_wrapper(&mut path, &board);
        results.push(path.iter().map(|x| x.0).collect::<HashSet<Point>>().len());
    }
    for x in 0..board.width {
        //relying on the order here!
        let start = Point::new(x, 0);
        let mut path = vec![(start, Direction::Down)];
        recursive_wrapper(&mut path, &board);
        results.push(path.iter().map(|x| x.0).collect::<HashSet<Point>>().len());
        let start = Point::new(x, board.height - 1);
        let mut path = vec![(start, Direction::Up)];
        recursive_wrapper(&mut path, &board);
        results.push(path.iter().map(|x| x.0).collect::<HashSet<Point>>().len());
    }

    *results.iter().max().unwrap() as i64
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 46;
    const PART_ONE_TEST: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    const PART_TWO_ANSWER: i64 = 51;
    const PART_TWO_TEST: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn fn_check_get_tile() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let input = Input::new(input);
        let lines = input.get_data().lines();
        let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
        let board = build_board(lines);
        assert_eq!(None, board.get_tile(Point::new(10000, 10000)));
    }

    #[test]
    #[should_panic]
    fn panics_char() {
        let input = "123";
        let result = Tile::from_char('X');
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }

    // #[test]
    // fn one_real() {
    //     let answer = part_one();
    //     assert_eq!(answer, answer);
    // }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
    // #[test]
    // fn two_real(){
    //     let answer = part_two();
    //     assert_eq!(answer,answer);

    // }
}
