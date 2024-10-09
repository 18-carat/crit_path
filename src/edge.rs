use crate::vertex::Vertex;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Edge {
    pub name: String,
    pub start: Vertex,
    pub end: Vertex,
    pub dist: u8,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }

        let tsv = s.split('\t').collect::<Vec<&str>>();
        let name = tsv[0].to_owned();
        let start = tsv[1].parse().unwrap();
        let end = tsv[2].parse().unwrap();
        let dist = tsv[3].parse().unwrap();

        Ok(Self {
            name,
            start,
            end,
            dist,
        })
    }
}

pub fn parse_file(path: &str) -> Vec<Edge> {
    let contents = read_to_string(path).unwrap();
    contents.lines().filter_map(|s| s.parse().ok()).collect()
}
