use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
pub fn the_day() -> u32 {
    10
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    Vertical,
    Horizontal,
    NtoE,
    NtoW,
    StoW,
    StoE,
    Ground,
    Start,
}
impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            'L' => TileType::NtoE,
            'J' => TileType::NtoW,
            '7' => TileType::StoW,
            'F' => TileType::StoE,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            _ => panic!("Unknown tile: {}", c),
        }
    }
    fn to_char(self) -> char {
        match self {
            TileType::Vertical => '│',
            TileType::Horizontal => '─',
            TileType::NtoE => '└',
            TileType::NtoW => '┘',
            TileType::StoW => '┐',
            TileType::StoE => '┌',
            TileType::Ground => '.',
            TileType::Start => 'S',
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    x: i64,
    y: i64,
}
impl Tile {
    fn new(tile_type: TileType, x: i64, y: i64) -> Tile {
        Tile { tile_type, x, y }
    }

    fn get_next_tile(&self) -> Option<((i64, i64), (i64, i64))> {
        match self.tile_type {
            TileType::Vertical => Some(((self.x, self.y + 1), (self.x, self.y - 1))),
            TileType::Horizontal => Some(((self.x - 1, self.y), (self.x + 1, self.y))),
            TileType::NtoE => Some(((self.x + 1, self.y), (self.x, self.y - 1))),
            TileType::StoW => Some(((self.x - 1, self.y), (self.x, self.y + 1))),
            TileType::NtoW => Some(((self.x - 1, self.y), (self.x, self.y - 1))),
            TileType::StoE => Some(((self.x + 1, self.y), (self.x, self.y + 1))),
            _ => None,
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    width: i64,
    height: i64,
    start: (i64, i64),
}
impl Board {
    fn new() -> Board {
        Board {
            tiles: Vec::new(),
            width: 0,
            height: 0,
            start: (-1, -1),
        }
    }
    fn get_tile(&self, x: i64, y: i64) -> Option<&Tile> {
        if y < self.height && y >= 0 && x < self.width && x >= 0 {
            return Some(&self.tiles[y as usize][x as usize]);
        }
        None
    }

    fn get_next_from_start(&self) -> Vec<&Tile> {
        let left = self.get_tile(self.start.0 - 1, self.start.1);
        let right = self.get_tile(self.start.0 + 1, self.start.1);
        let top = self.get_tile(self.start.0, self.start.1 - 1);
        let bottom = self.get_tile(self.start.0, self.start.1 + 1);
        let mut output: Vec<&Tile> = Vec::new();
        if let Some(tile) = left {
            if tile.tile_type == TileType::Horizontal
                || tile.tile_type == TileType::NtoE
                || tile.tile_type == TileType::StoE
            {
                output.push(tile);
            }
        }
        if let Some(tile) = right {
            if tile.tile_type == TileType::Horizontal
                || tile.tile_type == TileType::NtoW
                || tile.tile_type == TileType::StoW
            {
                output.push(tile);
            }
        }
        if let Some(tile) = top {
            if tile.tile_type == TileType::Vertical
                || tile.tile_type == TileType::StoW
                || tile.tile_type == TileType::StoE
            {
                output.push(tile);
            }
        }
        if let Some(tile) = bottom {
            if tile.tile_type == TileType::Vertical
                || tile.tile_type == TileType::NtoE
                || tile.tile_type == TileType::NtoW
            {
                output.push(tile);
            }
        }
        output
    }

    #[allow(unused)]
    fn print_board(&self) {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                print!("{}", tile.tile_type.to_char());
            }
            println!();
        }
    }

    #[allow(unused)]
    #[cfg(not(tarpaulin_include))]
    fn print_board_with_mask(&self, mask: &[(i64, i64)]) {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                let location = (tile.x, tile.y);
                if mask.contains(&location) {
                    print!("X");
                    continue;
                }
                print!("{}", tile.tile_type.to_char());
            }
            println!();
        }
    }
}

fn build_board(lines: Vec<&str>) -> Board {
    let mut board = Board::new();

    // Build the board
    for (y, line) in lines.iter().enumerate() {
        let line = line.trim();
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let tile_type = TileType::from_char(c);
            if let TileType::Start = tile_type {
                board.start = (x as i64, y as i64);
            }
            let tile = Tile::new(tile_type, x as i64, y as i64);
            row.push(tile);
        }
        board.width = row.len() as i64;
        board.tiles.push(row);
    }
    board.height = board.tiles.len() as i64;
    board
}

pub fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let board = build_board(lines);

    //build the loop
    let current_tile = board.get_next_from_start();
    let mut current_tile = current_tile[0];
    let mut last_tile = board.start;
    let mut steps = 0;
    loop {
        let next_tiles = current_tile.get_next_tile();
        let next_tile = if let Some((a, b)) = next_tiles {
            if a == last_tile {
                b
            } else {
                a
            }
        } else {
            panic!("No next tile")
        };
        last_tile = (current_tile.x, current_tile.y);
        current_tile = board.get_tile(next_tile.0, next_tile.1).unwrap();
        steps += 1;
        if current_tile.tile_type == TileType::Start {
            break;
        }
    }
    if steps % 2 == 0 {
        steps / 2
    } else {
        steps / 2 + 1
    }
}

pub fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let board = build_board(lines);

    //build the loop
    let current_tile = board.get_next_from_start();
    let mut current_tile = current_tile[0];
    let mut last_tile = board.start;

    let mut points: Vec<(i64, i64)> = [(board.start.0, board.start.1)].to_vec();
    loop {
        let next_tiles = current_tile.get_next_tile();
        let next_tile = if let Some((a, b)) = next_tiles {
            if a == last_tile {
                b
            } else {
                a
            }
        } else {
            panic!("No next tile")
        };
        last_tile = (current_tile.x, current_tile.y);
        points.push((current_tile.x, current_tile.y));
        //println!("Next tiles: {:?}; next tile: {:?}", next_tiles, next_tile);
        current_tile = board.get_tile(next_tile.0, next_tile.1).unwrap();
        if current_tile.tile_type == TileType::Start {
            break;
        }
    }

    let mut area = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        area = area + (points[i].0 * points[j].1) - (points[i].1 * points[j].0);
    }
    area = area.abs() / 2;
    area.abs() + 1 - ((1 + points.len() as i64) / 2)
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 80;
    const PART_ONE_TEST: &str = "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    const PART_ONE_ANSWER_TWO: i64 = 79;
    const PART_ONE_TEST_TWO: &str = "FF7SSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    const PART_TWO_ANSWER: i64 = 8;
    const PART_TWO_TEST: &str = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    const PANIC_ONE_TEST: &str = "..F7.
    F-J|.
    SL.L7
    |F-|J
    LJ...";

    const PANIC_TWO_TEST: &str = "..F7.
    .FJ|.
    SJ.L7
    |F-|J
    LJ...";

    #[test]
    #[should_panic]
    fn one_invalid_input() {
        let input = ProblemInput::String(PANIC_ONE_TEST.trim());
        do_part_one(Input::new(input));
    }
    #[test]
    #[should_panic]
    fn two_invalid_input() {
        let input = ProblemInput::String(PANIC_TWO_TEST.trim());

        do_part_two(Input::new(input));
    }
    #[test]
    #[should_panic]
    fn fn_invalid_char() {
        TileType::from_char('X');
    }

    #[test]
    fn fn_build_board() {
        let input = ProblemInput::String(PART_ONE_TEST.trim());
        let input = Input::new(input);
        let lines = input.get_data().lines();
        let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
        let mut board = build_board(lines);
        let res = board.get_next_from_start();
        println!("");
        board.print_board();
        println!("Res: {:?}", res);
        assert!(res.iter().filter(|x| x.x == 3 && x.y == 0).count() == 1);
        assert!(res.iter().filter(|x| x.x == 4 && x.y == 1).count() == 1);
    }

    #[test]
    fn fn_next_tile() {
        let input = ProblemInput::String(PART_ONE_TEST.trim());
        let input = Input::new(input);
        let lines = input.get_data().lines();
        let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
        let mut board = build_board(lines);
        let tile = Tile::new(TileType::NtoE, 1, 1);
        let res = tile.get_next_tile().unwrap();
        assert_eq!(res.0, (2, 1));
        assert_eq!(res.1, (1, 0));
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST.trim());
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);

        let input = ProblemInput::String(PART_ONE_TEST_TWO.trim());
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER_TWO);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let input = Input::new(input);
        let result = do_part_two(input);
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
