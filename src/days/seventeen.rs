use crate::utils::{Input, ProblemInput};
use pathfinding::prelude::dijkstra;
use std::hash::{Hash, Hasher};
use std::{ io::BufRead, time::Instant};


#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    17
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

////////////////////////////////////////////////////////////////
////////////////////// New attempt /////////////////////////////
////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Board {
    width: u32,
    height: u32,
    values: Vec<Vec<u32>>,
}
fn build_values(lines: Vec<&str>) -> Vec<Vec<u32>> {
    let mut values: Vec<Vec<u32>> = vec![vec![0; lines[0].len()]; lines.len()];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            values[y][x] = c.to_digit(10).unwrap();
        }
    }
    values
}
fn get_next_nodes(
    (x, y): (u32, u32),
    board: &Board,
    direction_in: Direction,
) -> Vec<((u32, u32), Direction)> {
    let mut neighbors: Vec<((u32, u32), Direction)> = Vec::new();
    if x > 0 && Direction::Right != direction_in {
        neighbors.push(((x - 1, y), Direction::Left));
    }
    if y > 0 && Direction::Down != direction_in {
        neighbors.push(((x, y - 1), Direction::Up));
    }
    if x < board.width  - 1 && Direction::Left != direction_in {
        neighbors.push(((x + 1, y), Direction::Right));
    }
    if y < board.height  - 1 && Direction::Up != direction_in {
        neighbors.push(((x, y + 1), Direction::Down));
    }
    neighbors
}

////////////////////////////////////////////////////////////////
////////////////////// New attempt /////////////////////////////
////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RouteNode {
    x: u32,
    y: u32,
    cost: u32,
    direction: Direction,
    step_in_direction: i32,
}

impl RouteNode {
    fn successors_part_two(&self, board: &Board, max: i32, min: i32) -> Vec<(RouteNode, u32)> {
        let next_nodes = get_next_nodes((self.x, self.y), board, self.direction);
        let mut results: Vec<(RouteNode, u32)> = Vec::new();
        //println!("Node: {:?}", &self);

        for (next_node, dir) in next_nodes {
            //println!("\tNext Node: {:?}", next_node);
            let mut next_node = RouteNode {
                x: next_node.0,
                y: next_node.1,
                direction: dir,
                cost: board.values[next_node.1 as usize][next_node.0 as usize],
                step_in_direction: self.step_in_direction,
            };
            if self.x == 0 && self.y == 0 {
            } else {
                next_node.step_in_direction += 1;
            }

            let change_dir = self.direction != dir;

            if (change_dir && self.step_in_direction < min ) ||(!change_dir && self.step_in_direction == max){
                continue;
            }else if change_dir {
                next_node.step_in_direction = 0;
            }

            results.push((next_node, next_node.cost));
        }
        results
    }
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
// impl Ord for RouteNode {
//     fn cmp(&self, other: &Self) -> Ordering {
//         // Notice that the we flip the ordering on costs.
//         // In case of a tie we compare positions - this step is necessary
//         // to make implementations of `PartialEq` and `Ord` consistent.
//         self
//             .cost
//             .cmp(&self.cost)
//             .then_with(|| (self.x + self.y).cmp(&(other.x + other.y)))
//     }
// }

// // `PartialOrd` needs to be implemented as well.
// impl PartialOrd for RouteNode {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
impl Hash for RouteNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.direction.hash(state);
        self.step_in_direction.hash(state);
    }
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let values = build_values(lines.clone());
    let board = &Board {
        width: lines[0].len() as u32,
        height: lines.len() as u32,
        values: values.clone(),
    };
    do_dijkstra(board, 2, 0)
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let values = build_values(lines.clone());
    let board = &Board {
        width: lines[0].len() as u32,
        height: lines.len() as u32,
        values: values.clone(),
    };
    do_dijkstra(board, 9, 3)
}

fn do_dijkstra(board: &Board, max: i32, min: i32) -> i64 {
    // let initial_node = RouteNode {
    //     x: 0,
    //     y: 0,
    //     cost: 0,
    //     direction: Direction::Right,
    //     step_in_direction: 0,
    // };
    let initial_node_right = RouteNode {
        x: 0,
        y: 0,
        cost: 0,
        direction: Direction::Right,
        step_in_direction: 0,
    };

    let result_right = dijkstra(
        &initial_node_right,
        |node| node.successors_part_two(board, max, min),
        |p| {
            p.x == board.width - 1
                && p.y == board.height - 1
                && p.step_in_direction >= min
                && p.step_in_direction <= max
        },
    );    

    let initial_node_down = RouteNode {
        x: 0,
        y: 0,
        cost: 0,
        direction: Direction::Down,
        step_in_direction: 0,
    };

    let result_down = dijkstra(
        &initial_node_down,
        |node| node.successors_part_two(board, max, min),
        |p| {
            p.x == board.width - 1
                && p.y == board.height - 1
                && p.step_in_direction >= min
                && p.step_in_direction <= max
        },
    );
    let result = result_right.unwrap().1.min(result_down.unwrap().1);  

    // let mut printer: Vec<Vec<String>> = board
    //     .values
    //     .iter()
    //     .map(|x| x.iter().map(|_x| '.'.to_string()).collect())
    //     .collect();

    // for row in result.0 {
    //     printer[row.y as usize][row.x as usize] = "#".to_string();
    // }
    // for row in printer {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }
    // println!();
    result as i64
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 102;
    const PART_ONE_TEST: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const PART_TWO_ANSWER: i64 = 94;
    const PART_TWO_TEST: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const PART_TWO_ANSWER_TWO: i64 = 71;
    const PART_TWO_TEST_TWO: &str = "111111111111
999999999991
999999999991
999999999991
999999999991";

//     const PART_TWO_ANSWER_THREE: i64 = 18;
//     const PART_TWO_TEST_THREE: &str = "111111111119999
// 999999999919999
// 999999999919999
// 999999999919999
// 999999999911111";


    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }
    #[test]
    fn one_real() {
        let (result, dur) = part_one();
        assert_eq!(result, 771);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        //println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);

        let input = ProblemInput::String(PART_TWO_TEST_TWO);
        let result = do_part_two(Input::new(input));
        //println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER_TWO);


    }

    #[test]
    fn two_real() {
        let (result, dur) = part_two();
        assert_eq!(result, 930);
    }
}
