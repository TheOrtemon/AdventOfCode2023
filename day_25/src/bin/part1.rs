use std::collections::HashMap;

use petgraph::{
    Graph, 
    visit::{EdgeRef, Dfs}, 
    algo::connected_components
};
use rustworkx_core::centrality::edge_betweenness_centrality;

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let (source, destinations_str) = line.split_once(": ").unwrap();
    (source, destinations_str.split_ascii_whitespace().collect())
}

fn find_popular_nodes(input: &str) -> usize {
    let mut pairs: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| parse_line(line))
        .collect();

    for value in pairs.clone().values() {
        for destination in value {
            pairs.entry(destination).or_default();
        }
    }
    let mut graph = Graph::new_undirected();
    let mut node_indexes = HashMap::new();
    for node_value in pairs.keys() {
        let node_index = graph.add_node(node_value.to_string());
        node_indexes.insert(node_value, node_index);
    }

    for node_value in pairs.keys() {
        let node_index = node_indexes.get(node_value).unwrap();
        for nb_node_value in pairs.get(node_value).unwrap() {
            let nb_node_index = node_indexes.get(nb_node_value).unwrap();
            graph.add_edge(*node_index, *nb_node_index, ());
        }
    }

    let edge_output = edge_betweenness_centrality(&graph, false, 200);
    let graph_clone = graph.clone();
    let edge_ids = graph_clone.edge_references().collect::<Vec<_>>();
    let mut output = edge_output.into_iter()
        .zip(edge_ids)
        .collect::<Vec<_>>();
    output.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let last = output.last().unwrap().1;
    let (node_a, node_b) =  (last.source(), last.target());
    for (_, edge) in output[output.len() - 3..].iter() {
        graph.remove_edge(edge.id());
    }
    assert_eq!(connected_components(&graph), 2);

    let mut counter_a = 0;
    let mut dfs_a = Dfs::new(&graph, node_a);
    while dfs_a.next(&graph).is_some() {
        counter_a += 1;
    }
    
    let mut counter_b = 0;
    let mut dfs_b = Dfs::new(&graph, node_b);
    while dfs_b.next(&graph).is_some() {
        counter_b += 1;
    }

    counter_a * counter_b
}

fn main() {
    let input = include_str!("./input.txt");
    let res = find_popular_nodes(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = "jqt: rhn xhk nvd
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
        assert_eq!(find_popular_nodes(test_input), 54);
    }
}