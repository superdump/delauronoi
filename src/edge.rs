use crate::vertex::Vertex2;
use crate::types::Position;

use nalgebra::Vector2;

pub struct Edge {
    pub origin: usize,
    pub destination: usize,
    pub face_left: Option<usize>,
    pub face_right: Option<usize>,
    pub edge_left_cw: Option<usize>,
    pub edge_left_ccw: Option<usize>,
    pub edge_right_cw: Option<usize>,
    pub edge_right_ccw: Option<usize>,
}

pub fn edge(origin: usize, destination: usize) -> Edge {
    Edge {
        origin,
        destination,
        face_left: None,
        face_right: None,
        edge_left_cw: None,
        edge_left_ccw: None,
        edge_right_cw: None,
        edge_right_ccw: None,
    }
}

impl Edge {
    // Negative on the left of the line
    // Note: does not check whether the ray from the point in a direction
    // perpendicular to the edge actually would intersect the edge
    pub fn signed_distance_to_point(&self, vertices: &Vec<Vertex2>, i: usize) -> f32 {
        let o = vertices[self.origin].position;
        let d = vertices[self.destination].position;
        let p = vertices[i].position;
        let line = d - o;
        return (line.y * p.x - line.x * p.y + d.x * o.y - d.y * o.x) / line.norm();
    }

    // Returns an f32 that is the signed distance from the line if an
    // intersection would occur, else None
    pub fn intersect_from_ray(&self, vertices: &Vec<Vertex2>, i: usize) -> Option<f32> {
        let p = vertices[i].position;
        let (o, d) = (vertices[self.origin].position, vertices[self.destination].position);
        let v1 = p - o;
        let v2 = d - o;
        let sign;
        let v3 = if cross(&v1, &v2) >= 0f32 {
            sign = 1f32;
            Vector2::<f32>::new(-v2.y, v2.x).normalize()
        } else {
            sign = -1f32;
            Vector2::<f32>::new(v2.y, -v2.x).normalize()
        };

        let denominator = v2.y * v3.x - v2.x * v3.y;
        if denominator.abs() > 1e-6_f32 {
            let t = (v3.x * (p.y - o.y) + v3.y * (o.x - p.x)) / denominator;
            let s = (v2.x * (p.y - o.y) + v2.y * (o.x - p.x)) / denominator;
            if 0f32 <= t && t <= 1f32 {
                return Some(sign * s);
            }
        }
        None
    }
}

fn cross(a: &Vector2<f32>, b: &Vector2<f32>) -> f32 {
    (a.x * b.y) - (a.y * b.x)
}
