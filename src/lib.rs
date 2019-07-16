pub use nalgebra::*;

pub type Vertex2 = Point2<f32>;
pub type Vertex2Array = Vec<Vertex2>;

pub fn v2(x: f32, y: f32) -> Vertex2 {
    Point2::<f32>::new(x, y)
}

pub fn v2_array() -> Vertex2Array {
    Vec::<Vertex2>::new()
}

pub fn v2_array_with_capacity(capacity: usize) -> Vertex2Array {
    Vec::<Vertex2>::with_capacity(capacity)
}
