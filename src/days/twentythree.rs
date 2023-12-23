use petgraph::stable_graph::{self, NodeIndex, StableGraph};

use crate::utils::{Input, ProblemInput};
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
    time::Instant,
};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    23
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    Empty,
    Slope(Direction),
    Forest,
}
impl TileType {
    fn from_char(c: char) -> TileType {
        match c {
            '.' => TileType::Empty,
            '^' => TileType::Slope(Direction::Up),
            'v' => TileType::Slope(Direction::Down),
            '<' => TileType::Slope(Direction::Left),
            '>' => TileType::Slope(Direction::Right),
            '#' => TileType::Forest,
            _ => panic!("Invalid tile type"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tile {
    x: i64,
    y: i64,
    tile_type: TileType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Board {
    width: i64,
    height: i64,
}
impl Tile {
    fn get_next(&self, board: &Board) -> Option<Vec<(i64, i64)>> {
        let mut res = Vec::new();
        if self.x > 0
            && (self.tile_type == TileType::Empty
                || self.tile_type == TileType::Slope(Direction::Left))
        {
            res.push((self.x - 1, self.y));
        }
        if self.x < board.width
            && (self.tile_type == TileType::Empty
                || self.tile_type == TileType::Slope(Direction::Right))
        {
            res.push((self.x + 1, self.y));
        }
        if self.y > 0
            && (self.tile_type == TileType::Empty
                || self.tile_type == TileType::Slope(Direction::Up))
        {
            res.push((self.x, self.y - 1));
        }
        if self.y < board.height
            && (self.tile_type == TileType::Empty
                || self.tile_type == TileType::Slope(Direction::Down))
        {
            res.push((self.x, self.y + 1));
        }
        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }
}

fn parse_input(
    input: Vec<&str>,
    do_part_two: bool,
) -> (
    stable_graph::StableGraph<Tile, i32>,
    HashMap<(i64, i64), NodeIndex>,
) {
    // must be a stable graph as we are going to edit the graph later on
    let mut graph = stable_graph::StableGraph::new();
    let mut tiles: Vec<Tile> = Vec::new();

    let mut tile_map: HashMap<(i64, i64), TileType> = HashMap::new();

    // this is a lookup table for the nodes in the graph from idx to value
    let mut graph_lookup: HashMap<(i64, i64), NodeIndex> = HashMap::new();

    let board = Board {
        width: input[0].len() as i64,
        height: input.len() as i64,
    };

    // build tile map
    input.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let mut tile_type = TileType::from_char(c);

            // part two requires we remove th directional tiles
            if do_part_two && tile_type != TileType::Forest {
                tile_type = TileType::Empty;
            }
            let tile = Tile {
                x: x as i64,
                y: y as i64,
                tile_type,
            };
            tiles.push(tile);
            tile_map.insert((x as i64, y as i64), tile_type);
        })
    });

    // Build Node and
    tiles.iter().for_each(|tile| {
        if tile.tile_type == TileType::Forest {
            return;
        }
        let ndx = graph.add_node(*tile);
        graph_lookup.insert((tile.x, tile.y), ndx);
    });

    // Build Edges
    tiles.iter().for_each(|tile| {
        if tile.tile_type == TileType::Forest {
            return;
        }
        let ndx = graph_lookup.get(&(tile.x, tile.y)).unwrap();
        let next = tile.get_next(&board);
        if let Some(next) = next {
            next.iter().for_each(|(x, y)| {
                if let Some(next_ndx) = graph_lookup.get(&(*x, *y)) {
                    graph.add_edge(*ndx, *next_ndx, 1);
                }
            });
        }
    });

    (graph, graph_lookup)
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (graph, graph_lookup) = parse_input(lines, false);
    get_longest_path(graph, graph_lookup, false)
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (graph, graph_lookup) = parse_input(lines, true);
    get_longest_path(graph, graph_lookup, true)
}

fn get_longest_path(
    mut graph: StableGraph<Tile, i32>,
    graph_lookup: HashMap<(i64, i64), NodeIndex>,
    do_pruning: bool,
) -> i64 {
    // get the start and finish points
    let mut keys = graph_lookup
        .keys()
        .clone()
        .copied()
        .collect::<Vec<(i64, i64)>>();
    keys.sort_by(|a, b| a.1.cmp(&b.1));

    let start = keys.first().unwrap();
    let finish = keys.last().unwrap();

    let start_ndx = *graph_lookup.get(start).unwrap();
    let finish_ndx = *graph_lookup.get(finish).unwrap();

    // the vector of visit lengths (the results)
    let mut visit_lengths: Vec<i64> = vec![];
    if do_pruning {
        println!("Nodes pre pruning: {}", graph.node_count());
    }

    // this loop iterates over the graph and removes any nodes that have only two neighbors
    // this is skipped for part one as it is not suitable as it does not account for the type of the node
    // e.g. directed/empty etc, we could add in logic to check that but part one is already quick enough

    loop {
        if !do_pruning {
            break;
        }
        // get a node that has only two neighbors but isn't connected to the start or finish
        // this makes it easier as we aren't later going to have to reconnect the start and finish
        // we just add one to the answer to account for the edge between the start and finish
        // find is good as it returns the first node that matches the predicate
        let ndx = graph.node_indices().find(|ndx| {
            graph.neighbors(*ndx).count() == 2
                && !graph
                    .neighbors(*ndx)
                    .collect::<Vec<_>>()
                    .contains(&start_ndx)
                && !graph
                    .neighbors(*ndx)
                    .collect::<Vec<_>>()
                    .contains(&finish_ndx)
        });

        match ndx {
            None => break,
            Some(ndx) => {
                let neighbors = graph.neighbors(ndx).collect::<Vec<_>>();

                //check all three nodes are the same type

                // get the weights of the edges between the neighbors and the node and then insert an edge between the neighbors
                // with the sum of the weights to skip the node
                let edge_weight_a_to_b = graph
                    .edges_connecting(neighbors[0], ndx)
                    .next()
                    .unwrap()
                    .weight()
                    + graph
                        .edges_connecting(ndx, neighbors[1])
                        .next()
                        .unwrap()
                        .weight();

                // same again in the other direction
                let edge_weight_b_to_a = graph
                    .edges_connecting(neighbors[1], ndx)
                    .next()
                    .unwrap()
                    .weight()
                    + graph
                        .edges_connecting(ndx, neighbors[0])
                        .next()
                        .unwrap()
                        .weight();

                // add these edges to the graph
                graph.add_edge(neighbors[0], neighbors[1], edge_weight_a_to_b);
                graph.add_edge(neighbors[1], neighbors[0], edge_weight_b_to_a);

                // now delete the node (this deletes the edges associated with the node as well)
                graph.remove_node(ndx);

                // this all only works because we are using a stable graph, if we were using a normal graph the node indices would change
            }
        }
    }

    if do_pruning {
        println!("Nodes post pruning: {}", graph.node_count());
    }
    // Create deques of nodes to visit and visited nodes
    let mut to_visit = VecDeque::new();
    let mut visited = VecDeque::new();

    // Add the start node to the to_visit deque
    to_visit.push_back((start_ndx, start_ndx));

    // odd trick, can't work out how to get the type for the VecDeque
    visited.push_back(start_ndx);
    visited.pop_back();

    while let Some((next, parent)) = to_visit.pop_front() {
        // first check if we have finished
        if next == finish_ndx {
            let mut cost = 0;

            // add the edge between the the tiles in the path, note the zip trick to pair up the nodes
            visited
                .iter()
                .zip(visited.iter().skip(1))
                .for_each(|(a, b)| {
                    let edge_weight = *graph.edges_connecting(*a, *b).next().unwrap().weight();
                    cost += edge_weight;
                });

            // add the edge between the final tile and the finish
            cost += *graph
                .edges_connecting(parent, next)
                .next()
                .unwrap()
                .weight();

            // add the cost to the visit lengths
            visit_lengths.push(cost as i64);
        } else {
            // if we havent reached the end the check if the current node's parent (parenrt) is in the visited list
            // if it isn't then it says the DFS has popped back up the tree and thus we need to prune the visited list
            while let Some(ndx) = visited.pop_back() {
                if ndx == parent {
                    visited.push_back(parent);
                    break;
                }
            }

            // add the current node to the visited list
            visited.push_back(next);

            // now add the children of the current node to the to_visit list but only if they aren't already in the visited list
            for child in graph.neighbors(next) {
                if !visited.contains(&child) {
                    to_visit.push_front((child, next));
                }
            }
        }
    }

    // get the maximal visit length
    visit_lengths.sort();
    *visit_lengths.last().unwrap()
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 94;
    const PART_ONE_TEST: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    const PART_TWO_ANSWER: i64 = 154;
    const PART_TWO_TEST: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    #[should_panic]
    fn panics() {
        let input = "###!##";
        input.chars().for_each(|c| {
            let _ = TileType::from_char(c);
        });
    }

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
