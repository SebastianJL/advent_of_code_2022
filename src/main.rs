use std::{collections::HashMap, error::Error, time::Instant};

use regex::Regex;

#[derive(Debug)]
struct Node {
    flow: u32,
    next: Vec<usize>,
}
type Graph<'a> = Vec<Node>;
type State = (usize, u64, u32);

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let (start_node, graph) = parse(&input);

    let opened = 0_u64;
    let depth = 30;
    let mut cache = HashMap::new();
    // println!("{graph:?}");
    let res = max_flow(start_node, &graph, opened, depth, &mut cache);
    dbg!(res);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn max_flow<'a>(
    start_node: usize,
    graph: &'a Graph,
    mut open: u64,
    depth: u32,
    cache: &mut HashMap<State, u32>,
) -> u32 {
    // If result has been cached return it directly.
    let state = (start_node, open, depth);
    if let Some(res) = cache.get(&state) {
        return *res;
    }
    if depth < 2 {
        return 0;
    }

    let my_node = start_node;
    let Node {
        flow: my_flow,
        next: next_nodes,
    } = graph.get(my_node).unwrap();


    // Open your own valve then move on.
    let mut option_1 = 0;
    if *my_flow != 0 && (open & (1 << my_node) == 0) {
        open |= 1 << my_node;
        option_1 = my_flow * (depth - 1)
            + next_nodes
                .iter()
                .map(|node| max_flow(*node, graph, open, depth - 2, cache))
                .max()
                .unwrap();
    }

    // Don't open your own valve. Move on directly.
    let option_2 = next_nodes
        .iter()
        .map(|node| max_flow(*node, graph, open, depth - 1, cache))
        .max()
        .unwrap();

    let res = option_1.max(option_2);
    cache.insert(state, res);
    res
}

fn parse(input: &str) -> (usize, Graph) {
    let re_nodes = Regex::new(r"[A-Z][A-Z]").unwrap();
    let re_flow = Regex::new(r"\d+").unwrap();
    let mut hm = HashMap::new();

    for line in input.lines() {
        let fma = re_flow.find(line).unwrap();
        let flow = line[fma.start()..fma.end()].parse::<u32>().unwrap();
        let mut nodes = re_nodes
            .find_iter(line)
            .map(|ma| &line[ma.start()..ma.end()])
            .collect::<Vec<&str>>();
        let key = nodes.remove(0);
        hm.insert(key, (flow, nodes));
    }

    let mut what: Vec<_> = hm.into_iter().collect();
    // Sort what by flow.
    what.sort_by(|(_, (flow_a, _)), (_, (flow_b, _))| flow_b.cmp(flow_a));
    let graph: Vec<_> = what
        .iter()
        .cloned()
        .map(|(_, (flow, nodes))| Node {
            flow,
            next: nodes
                .into_iter()
                .map(|node_a| {
                    what.iter()
                        .position(|(node_b, _)| *node_b == node_a)
                        .unwrap()
                })
                .collect(),
        })
        .collect();
    let start_node = what.iter().position(|(node, _)| *node == "AA").unwrap();
    (start_node, graph)
}
