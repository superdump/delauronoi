pub struct Edge {
    vertex_origin: usize,
    vertex_destination: usize,
    face_left: Option<usize>,
    face_right: Option<usize>,
    edge_left_cw: Option<usize>,
    edge_left_ccw: Option<usize>,
    edge_right_cw: Option<usize>,
    edge_right_ccw: Option<usize>,
}
