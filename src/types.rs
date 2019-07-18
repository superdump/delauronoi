use nalgebra::Point2;
use crate::vertex::Vertex2;

pub type Position = Point2<f32>;
pub type Vertex2Array = Vec<Vertex2>;

pub fn position(x: f32, y: f32) -> Position {
    Point2::<f32>::new(x, y)
}

pub fn vertex2_array() -> Vertex2Array {
    Vec::<Vertex2>::new()
}

pub fn vertex2_array_with_capacity(capacity: usize) -> Vertex2Array {
    Vec::<Vertex2>::with_capacity(capacity)
}
