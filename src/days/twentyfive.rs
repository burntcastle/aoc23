use crate::utils::{Input, ProblemInput};
use petgraph::prelude::*;
use petgraph::stable_graph::StableGraph;
use rand::seq::IteratorRandom;
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    25
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
    (0, now.elapsed())
}

// takes the input and parses it into a Undirected StableGraph and a hashmap of node weights to node indexes
fn parse(input: Vec<&str>) -> (StableGraph<&str, i64, Undirected>, HashMap<&str, NodeIndex>) {
    let mut graph: StableGraph<&str, i64, Undirected> = StableGraph::default();
    let mut node_lookup: HashMap<&str, NodeIndex> = HashMap::new();
    for line in input {
        // string parsing;  #TODO: use nom crate for proper parsing
        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let deps = parts.next().unwrap().split(' ');
        let deps: Vec<&str> = deps.map(|x| x.trim()).collect();

        if !node_lookup.contains_key(name) {
            let node = graph.add_node(name);
            node_lookup.insert(name, node);
        }
        for dep in deps {
            if !node_lookup.contains_key(dep) {
                let node = graph.add_node(dep);
                node_lookup.insert(dep, node);
            }
            graph.add_edge(
                *node_lookup.get(name).unwrap(),
                *node_lookup.get(dep).unwrap(),
                1,
            );
        }
    }
    (graph, node_lookup)
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (graph, _ndx_lookup) = parse(lines);

    // we loop through the graph until we get a cut with 3 edges
    loop {
        let (cuts, left, right) = karger_min_cut(&graph);
        if cuts == 3 {
            return left * right;
        }
    }
}

// from here: https://www.cs.princeton.edu/~hy2/teaching/fall22-cos521/notes/lec1.pdf
// and here: https://www.geeksforgeeks.org/kargers-algorithm-for-minimum-cut-set-1-introduction-and-implementation/
fn karger_min_cut(graph: &StableGraph<&str, i64, Undirected>) -> (i64, i64, i64) {
    // fill the component size map with single values for each node
    let mut component_size: HashMap<NodeIndex, i64> = HashMap::new();
    for ndx in graph.node_indices() {
        component_size.insert(ndx, 1);
    }
    // create a mutable clone of the graph
    let mut graph = graph.clone();
    loop {
        // if we get down to out last two nodes we're done
        if graph.node_count() == 2 {
            break;
        }
        // pick a random node
        let n1 = graph
            .node_indices()
            .choose(&mut rand::thread_rng())
            .unwrap();
        // pick a random neighbour of that node, this is the node we're merging into
        let n2 = graph.neighbors(n1).choose(&mut rand::thread_rng()).unwrap();
        // collect the edges for the node; note it needs to be edges and not neighbours, if it's neigbours
        // we'll loose some edges as they get doubled up to represent parallel paths.
        let n2_edges = graph.edges(n2).map(|x| x.id()).collect::<Vec<EdgeIndex>>();
        // loop through the edges of the node we're merging into
        for n2_edge in n2_edges {
            // get the nodes at the end of the edge
            let (n3, n4) = graph.edge_endpoints(n2_edge).unwrap();
            // check that it isn't a self loop or a loop to the node we're merging into
            if n3 == n1 || n4 == n1 {
                continue;
            } else {
                // find the edge to remove and remove it
                let edge_to_remove = graph.find_edge(n3, n4).unwrap();
                graph.remove_edge(edge_to_remove).unwrap();

                // this gets round the fact that we don't know which of n3 or n4 is the node we want to keep
                // so we add both but we'll delete the unwanted node shortly so it doesn't matter
                graph.add_edge(n1, n3, 1);
                graph.add_edge(n1, n4, 1);
            }
        }
        // Keep track of the number of nodes in each component
        component_size.insert(
            n1,
            component_size.get(&n1).unwrap() + component_size.get(&n2).unwrap(),
        );
        // and remove the merged node
        graph.remove_node(n2);
        component_size.remove(&n2);
    }
    // get the two nodes 
    let node_indexs = graph.node_indices().collect::<Vec<NodeIndex>>();
    // shouldn't need to check for self loops, but just in case
    assert_eq!(node_indexs.len(), 2);
    let node_1 = node_indexs.first().unwrap();
    let node_2 = node_indexs.last().unwrap();

    return (
        graph.edges_connecting(*node_1, *node_2).count() as i64,
        *component_size.get(node_1).unwrap(),
        *component_size.get(node_2).unwrap(),
    );
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 54;
    const PART_ONE_TEST: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    const PART_TWO_ANSWER: i64 = 0;
    const PART_TWO_TEST: &str = "";

    #[test]
    #[should_panic]
    fn panics() {
        parse(vec![""]);
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }
}
