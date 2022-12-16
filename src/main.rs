use std::{
    collections::{HashMap},
    error::Error,
    time::Instant,
};

use regex::Regex;

type Graph<'a> = HashMap<&'a str, (u32, Vec<&'a str>)>;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let input = read();
    let graph = parse(&input);

    let start_node = "AA";
    let mut visited = Vec::new();
    let depth = 30;
    let res = max_flow(start_node, &graph, &mut visited, depth);
    dbg!(res);

    let runtime = start.elapsed();
    dbg!(runtime);
    Ok(())
}

fn read() -> String {
    std::fs::read_to_string("input.txt").expect("File not found.")
}

fn max_flow<'a>(node: &'a str, g: &'a Graph, visited: &mut Vec<&'a str>, depth: u32) -> u32 {
    let my_node = node;
    let (my_flow, next_nodes) = g.get(my_node).unwrap();

    if depth == 2 {
        return my_flow * (depth - 1);
    } else if depth < 2 {
        return 0;
    }
    // dbg!(visited.len());
    if g.len() == visited.len() {
        return 0;
    }

    // Open your own valve then move on.
    let mut option_1 = 0;
    if !visited.contains(&my_node) && depth >= 2 && *my_flow != 0 {
        visited.push(my_node);
        // dbg!(&visited);
        option_1 = my_flow * (depth - 1)
            + next_nodes
                .iter()
                .map(|node| max_flow(node, g, visited, depth - 2))
                .max()
                .unwrap();
        visited.pop();
    }

    // Don't open your own valve. Move on directly.
    let option_2 = next_nodes
        .iter()
        .map(|node| max_flow(node, g, visited, depth - 1))
        .max()
        .unwrap();

    return option_1.max(option_2);
}

fn parse(input: &str) -> Graph {
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

    hm
}
