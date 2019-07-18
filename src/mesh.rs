use crate::edge::*;
use crate::face::*;
use crate::vertex::*;

pub struct Mesh {
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub vertices: Vec<Vertex2>,
}

pub fn mesh() -> Mesh {
    Mesh {
        edges: Vec::new(),
        faces: Vec::new(),
        vertices: Vec::new(),
    }
}

impl Mesh {
    pub fn add_edge(&mut self, e: Edge) {
        self.edges.push(e);
    }

    pub fn add_face(&mut self, f: Face) {
        self.faces.push(f);
    }

    pub fn add_vertex(&mut self, v: Vertex2) {
        self.vertices.push(v);
    }
}
