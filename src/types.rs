use nalgebra::Point2;

pub type Position = Point2<f32>;

pub const EPSILON: f32 = 1e-6_f32;

pub fn position(x: f32, y: f32) -> Position {
    Point2::<f32>::new(x, y)
}
