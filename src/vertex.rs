use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vertex {
    pub idx: u8,
}

impl Vertex {
    pub fn new(idx: u8) -> Self {
        Self { idx }
    }
}

impl FromStr for Vertex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(idx) = s.parse() else {
            return Err(());
        };

        Ok(Self { idx })
    }
}
