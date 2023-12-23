use crate::utils::{Input, ProblemInput};
use kdam::tqdm;
use petgraph::graph::Graph;
use petgraph::prelude::*;

use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    21
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum LocationType {
    GardenPlot,
    Rock,
}
impl Display for LocationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationType::GardenPlot => write!(f, "."),
            LocationType::Rock => write!(f, "#"),
        }
    }
}

// fn print_garden(garden: &Vec<Vec<LocationType>>) {
//     for row in garden {
//         for item in row {
//             print!("{}", item);
//         }
//         println!();
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
struct Location {
    location: LocationType,
    x: i64,
    y: i64,
}
impl Location {
    fn new(location: LocationType, x: i64, y: i64) -> Self {
        Self { location, x, y }
    }
    fn xandy(&self) -> (i64, i64) {
        (self.x, self.y)
    }
    fn get_neighbours(&self) -> Vec<(i64, i64)> {
        vec![
            (self.x - 1, self.y),
            (self.x + 1, self.y),
            (self.x, self.y - 1),
            (self.x, self.y + 1),
        ]
    }
}
impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.location {
            LocationType::GardenPlot => write!(f, "({},{})", self.x, self.y),
            LocationType::Rock => write!(f, "#({},{})", self.x, self.y),
        }
    }
}
fn parse_input(input: Vec<&str>) -> (Vec<Vec<LocationType>>, (i64, i64)) {
    let mut location = (0, 0);
    let mut garden: Vec<Vec<LocationType>> = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<LocationType> = vec![];
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => row.push(LocationType::GardenPlot),
            '#' => row.push(LocationType::Rock),
            'S' => {
                row.push(LocationType::GardenPlot);
                location = (x as i64, y as i64);
            }
            _ => panic!("Invalid input"),
        });

        garden.push(row);
    }
    (garden, location)
}

fn build_graph(
    locations: Vec<Vec<LocationType>>,
) -> (Graph<Location, i64>, HashMap<(i64, i64), NodeIndex>) {
    let mut graph = Graph::<Location, i64>::new();
    let mut node_labels = HashMap::<(i64, i64), NodeIndex>::new();

    // add the nodes
    for (y, row) in locations.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let node = Location::new(*item, x as i64, y as i64);
            let ndx = graph.add_node(node);
            node_labels.insert(node.xandy(), ndx);
        }
    }

    //add the edges
    for (y, row) in locations.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let node = Location::new(*item, x as i64, y as i64);
            for neighbour in node.get_neighbours() {
                if let Some(ndx) = node_labels.get(&neighbour) {
                    if locations[neighbour.1 as usize][neighbour.0 as usize]
                        == LocationType::GardenPlot
                        && item == &LocationType::GardenPlot
                    {
                        graph.add_edge(*node_labels.get(&node.xandy()).unwrap(), *ndx, 1);
                    }
                    //graph.add_edge(*node_labels.get(&node.xandy()).unwrap(), *ndx, 1);
                }
            }
        }
    }
    (graph, node_labels)
}

fn do_part_one(input: Input) -> i64 {
    let target = 64;

    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();

    let (garden, location) = parse_input(lines);

    let (graph, node_labels) = build_graph(garden);

    let start = node_labels.get(&location).unwrap();

    let mut queue: VecDeque<NodeIndex> = VecDeque::new();
    let mut visit_costs: HashMap<NodeIndex, i64> = HashMap::new();
    queue.push_back(*start);
    visit_costs.insert(*start, 0);

    while let Some(ndx) = queue.pop_front() {
        let count = *visit_costs.get(&ndx).unwrap();
        let neighbours = graph.neighbors(ndx);
        let neighbours = neighbours.into_iter();

        for neighbour in neighbours {
            let mut update = count + 1;
            let mut do_update = false;
            if visit_costs.contains_key(&neighbour) {
                if visit_costs.get(&neighbour).unwrap() > &(update) {
                    update = *visit_costs.get(&neighbour).unwrap();
                    do_update = true;
                }
            } else {
                do_update = true;
            }
            if do_update && count < target {
                visit_costs.insert(neighbour, update);
                queue.push_back(neighbour);
            }
        }
    }
    if target % 2 == 0 {
        //println!("Max: {}", max_count);
        visit_costs.values().filter(|x| (*x % 2 == 0)).count() as i64
    } else {
        visit_costs.values().filter(|x| (*x % 2 != 0)).count() as i64
    }
}

fn parse_input_growth(input: Vec<&str>, repititions: i64) -> (Vec<Vec<LocationType>>, (i64, i64)) {
    let mut location = (0, 0);
    let mut garden: Vec<Vec<LocationType>> = vec![];

    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<LocationType> = vec![];
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => row.push(LocationType::GardenPlot),
            '#' => row.push(LocationType::Rock),
            'S' => {
                row.push(LocationType::GardenPlot);
                location = (x as i64, y as i64);
            }
            _ => panic!("Invalid input"),
        });

        garden.push(row);
    }
    location = (
        location.0 + garden[0].len() as i64 * repititions,
        location.1 + garden.len() as i64 * repititions,
    );

    let mut extended_garden: Vec<Vec<LocationType>> = vec![];
    //extended_garden.extend(garden.clone());
    // make it wide
    for _j in 0..repititions {
        let mut tmp_extended_garden: Vec<Vec<LocationType>> = vec![];
        for row in &garden {
            let mut new_row = vec![];
            // the orginal row
            new_row.extend(row.clone());
            for _i in 0..repititions {
                new_row.extend(row.clone());
                new_row.extend(row.clone());
            }
            tmp_extended_garden.push(new_row);
        }
        if extended_garden.is_empty() {
            extended_garden.extend(tmp_extended_garden.clone());
        }
        extended_garden.extend(tmp_extended_garden.clone());
        extended_garden.extend(tmp_extended_garden.clone());
    }
    //println!("Garden size: {}x{}", extended_garden[0].len(), extended_garden.len());
    //print_garden(&extended_garden);
    (extended_garden, location)
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut x: Vec<i64> = vec![];
    let mut y: Vec<i64> = vec![];
    for &i in tqdm!((0..500).collect::<Vec<i64>>().iter()) {
        let width = lines[0].len() as i64;
        let mut n = 0;
        if i > width {
            n = (i - width) / width;
        }
        let result = calulcate_extended_map(lines.clone(), i, 2 + n);

        //println!("{}, {}", i,result);
        x.push(i);
        y.push(result);
    }

    println!("x = {:?}", x);
    println!("y = {:?}", y);

    100
}

fn calulcate_extended_map(lines: Vec<&str>, target: i64, repititions: i64) -> i64 {
    let (garden, location) = parse_input_growth(lines, repititions);

    let (graph, node_labels) = build_graph(garden);
    let start = node_labels.get(&location).unwrap();
    let mut queue: VecDeque<NodeIndex> = VecDeque::new();
    let mut visit_costs: HashMap<NodeIndex, i64> = HashMap::new();
    queue.push_back(*start);
    visit_costs.insert(*start, 0);
    let mut max_count = 0;
    //print!("Breadth First Search");
    while let Some(ndx) = queue.pop_front() {
        let count = *visit_costs.get(&ndx).unwrap();

        let neighbours = graph.neighbors(ndx);
        let neighbours = neighbours.into_iter();

        for neighbour in neighbours {
            let mut update = count + 1;
            let mut do_update = false;
            if visit_costs.contains_key(&neighbour) {
                if visit_costs.get(&neighbour).unwrap() > &(update) {
                    update = *visit_costs.get(&neighbour).unwrap();
                    do_update = true;
                }
            } else {
                do_update = true;
            }
            if do_update && count < target {
                visit_costs.insert(neighbour, update);
                queue.push_back(neighbour);
            }
            if update > max_count {
                max_count = update;
            }
        }
    }
    if target % 2 == 0 {
        //println!("Max: {}", max_count);
        visit_costs
            .clone()
            .values()
            .filter(|x| (*x % 2 == 0))
            .count() as i64
    } else {
        visit_costs
            .clone()
            .values()
            .filter(|x| (*x % 2 != 0))
            .count() as i64
    }
}
#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::sync::MutexGuard;

    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 42;
    const PART_ONE_TEST: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    const PART_ONE_TEST_SIMPLE: &str = "#####
##.##
##.##
##S..
#####";

    const PART_TWO_ANSWER: i64 = 42;
    const PART_TWO_TEST: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    #[should_panic]
    fn panics() {
        let input = " ...?....";
        let (result, (x, y)) = parse_input(input.lines().collect());
    }

    #[test]
    fn fn_parse() {
        let input = PART_ONE_TEST;
        let (result, (x, y)) = parse_input(input.lines().collect());

        assert_eq!(x, 5);
        assert_eq!(y, 5);
        assert!(result.len() == 11);
        assert_eq!(result[2][1], LocationType::Rock);
    }

    #[test]
    fn one_given() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }
    #[test]
    fn one_simple() {
        let input = ProblemInput::String(PART_ONE_TEST_SIMPLE);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, 3);
    }

    // #[test]
    // fn two() {
    //     let input = ProblemInput::String(PART_TWO_TEST);
    //     let result = do_part_two(Input::new(input));
    //     println!("Result: {}", result);
    //     assert_eq!(result, PART_TWO_ANSWER);
    // }
}
