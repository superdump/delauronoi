use crate::types::*;

pub struct Vertex2 {
    pub position: Position,
    edge: Option<usize>,
}

pub fn v2(x: f32, y: f32) -> Vertex2 {
    Vertex2 {
        position: position(x, y),
        edge: None,
    }
}
