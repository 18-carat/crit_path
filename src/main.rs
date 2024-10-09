#![forbid(unsafe_code)]
#![allow(dead_code)]

mod critical_path;
mod dependency;
mod edge;
mod graph;
mod vertex;

use critical_path::critical_path;
use vertex::Vertex;

fn main() {
    let edges = edge::parse_file("test.txt");
    let dist = critical_path(edges, Vertex::new(1));

    println!("{dist}");
}

#[test]
fn it_works() {
    let edges = edge::parse_file("tests/1.txt");
    let dist = critical_path(edges, Vertex::new(1));

    assert_eq!(dist, "A-C-F-I-J-K, 16");

    let edges = edge::parse_file("tests/2.txt");
    let dist = critical_path(edges, Vertex::new(1));

    assert_eq!(dist, "A-D-F-G-I-K-L, 48");

    let edges = edge::parse_file("tests/3.txt");
    let dist = critical_path(edges, Vertex::new(1));

    assert_eq!(dist, "B-E-H-J, 16");
}
