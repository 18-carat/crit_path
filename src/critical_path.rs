use crate::edge::Edge;
use crate::graph::DependencyGraph;
use crate::vertex::Vertex;
use std::collections::HashMap;

fn edges_to_hm(edges: Vec<Edge>) -> HashMap<Vertex, Vec<(Vertex, u8)>> {
    let mut graph: HashMap<Vertex, Vec<(Vertex, u8)>> = HashMap::new();

    for edge in edges {
        graph
            .entry(edge.start)
            .or_default()
            .push((edge.end, edge.dist));
    }

    graph
}

fn hm_to_graph(hm: HashMap<Vertex, Vec<(Vertex, u8)>>) -> DependencyGraph<Vertex> {
    let mut graph = DependencyGraph::new();

    for (start, neighbors) in hm {
        for &(end, _) in &neighbors {
            graph.depend_on(start, end);
        }
    }

    graph
}

fn vertices_to_edges(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Vec<Edge> {
    let mut ret = Vec::new();

    for window in vertices.windows(2) {
        if let [start, end] = window {
            let edge = edges
                .iter()
                .find(|e| e.start == *start && e.end == *end)
                .unwrap();

            ret.push(edge.clone());
        }
    }

    ret
}

pub fn critical_path(edges: Vec<Edge>, start: Vertex) -> String {
    let hm = edges_to_hm(edges.clone());
    let mut graph = hm_to_graph(hm.clone());
    let sorted_nodes = graph.sorted();
    let sorted_nodes = sorted_nodes.expect("Graphs must be acyclic");

    let mut distances = HashMap::new();
    let mut predecessors = HashMap::new();

    for node in sorted_nodes.iter() {
        distances.insert(*node, u8::MIN);
        predecessors.insert(*node, None);
    }

    distances.insert(start, 0);

    for node in sorted_nodes {
        if let Some(edges) = hm.get(&node) {
            for &(neighbor, weight) in edges {
                let new_distance = distances[&node] + weight;

                if new_distance > distances[&neighbor] {
                    distances.insert(neighbor, new_distance);
                    predecessors.insert(neighbor, Some(node));
                }
            }
        }
    }

    let max_distance = *distances.values().max().unwrap_or(&0);
    let mut critical_path = Vec::new();

    let end_node = distances
        .iter()
        .find(|&(_, &d)| d == max_distance)
        .map(|(v, _)| *v);

    let mut current = end_node;

    while let Some(node) = current {
        critical_path.push(node);
        current = predecessors[&node];
    }

    critical_path.reverse();

    let edges = vertices_to_edges(critical_path, edges)
        .iter()
        .map(|e| e.name.clone())
        .collect::<Vec<String>>()
        .join("-");

    format!("{edges}, {max_distance}")
}
