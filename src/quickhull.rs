use crate::edge::{edge, Edge};
use crate::hull::{hull, Hull};
use crate::mesh::{mesh, Mesh};
use crate::vertex::Vertex2;

pub fn quickhull(vertices: &Vec<Vertex2>) -> Result<Hull, ()> {
    if vertices.len() < 2 {
        return Err(());
    }
    let mut hull_vertices = Vec::new();
    let (left, right) = find_left_right(vertices);
    hull_vertices.push(left);
    hull_vertices.push(right);
    let e = edge(left, right);
    let e_rev = edge(right, left);
    // Partition the points into two sets, those to the right of the line
    // segment defined by the edge and those to the left
    let (s_right, s_left) = partition_points(vertices, &e, None);
    assert_eq!(s_right.len() + s_left.len(), vertices.len() - 2);
    find_hull(vertices, &s_right, &e, &mut hull_vertices)?;
    find_hull(vertices, &s_left, &e_rev, &mut hull_vertices)?;
    return hull_from_vertices(vertices, &hull_vertices);
}

fn find_left_right(vertices: &Vec<Vertex2>) -> (usize, usize) {
    let mut left: usize = 0;
    let mut right: usize = 0;
    vertices.iter().enumerate().for_each(|(i, v)| {
        if v.position.x < vertices[left].position.x {
            left = i;
        }
        if v.position.x > vertices[right].position.x {
            right = i;
        }
    });
    (left, right)
}

fn partition_points(vertices: &Vec<Vertex2>, e: &Edge, s_opt: Option<&Vec<(usize, f32)>>) -> (Vec<(usize, f32)>, Vec<(usize, f32)>) {
    let mut s_right = Vec::<(usize, f32)>::new();
    let mut s_left = Vec::<(usize, f32)>::new();
    let closure = |i: &usize| {
        if *i == e.origin || *i == e.destination {
            return;
        }
        let d = e.signed_distance_to_point(vertices, *i);
        if d >= 0f32 {
            s_right.push((*i, d));
        } else {
            s_left.push((*i, d));
        }
    };
    if let Some(s) = s_opt {
        s.iter().map(|(i, _)| *i).collect::<Vec<_>>().iter().for_each(closure);
    } else {
        (0..vertices.len()).into_iter().collect::<Vec<_>>().iter().for_each(closure);
    }
    (s_right, s_left)
}

fn find_hull(vertices: &Vec<Vertex2>, s: &Vec<(usize, f32)>, e: &Edge, hull: &mut Vec<usize>) -> Result<(), ()> {
    if s.len() < 1 {
        return Ok(());
    }
    // Find the point farthest from the edge
    let (mut dist_max, mut i_max) = (0f32, 0);
    for (i, dist) in s {
        let d_abs = dist.abs();
        if d_abs > dist_max {
            dist_max = d_abs;
            i_max = *i;
        }
    }
    // Insert the point between the edge origin and destination
    vec_insert_after_value(hull, &e.origin, i_max)?;
    // Partition the points to the right of e that are in s into those to the
    // right of the edges origin to max (the farthest point) and max to the
    // destination
    let (o_max, max_d) = (edge(e.origin, i_max), edge(i_max, e.destination));
    let (s1, s0) = partition_points(vertices, &o_max, Some(s));
    let (s2, _) = partition_points(vertices, &max_d, Some(&s0));
    // Find the hulls of these partitions
    find_hull(vertices, &s1, &o_max, hull)?;
    find_hull(vertices, &s2, &max_d, hull)?;
    Ok(())
}

fn vec_insert_after_value<T: Eq>(vec: &mut Vec<T>, after: &T, value: T) -> Result<(), ()> {
    let mut i_after = None;
    for (i, e) in vec.iter().enumerate() {
        if e == after {
            i_after = Some(i);
            break;
        }
    }
    if let Some(i_after) = i_after {
        vec_insert_after(vec, i_after, value);
        return Ok(());
    }
    Err(())
}

fn vec_insert_after<T>(vec: &mut Vec<T>, after: usize, value: T) {
    vec.push(value);
    vec[(after + 1)..].rotate_right(1);
}

fn hull_from_vertices(vertices: &Vec<Vertex2>, hull_vertices: &Vec<usize>) -> Result<Hull, ()> {
    if hull_vertices.len() < 2 {
        return Err(());
    }
    let mut mesh = mesh();
    let mut e_prev = None;
    for i in 0..hull_vertices.len() {
        let mut e = edge(hull_vertices[i], hull_vertices[(i + 1) % hull_vertices.len()]);
        if let Some(e_prev_i) = e_prev {
            let mut prev = &mut mesh.edges[e_prev_i];
            set_adjacent_edges(vertices, e_prev_i, &mut prev, i, &mut e);
        }
        e_prev = Some(mesh.add_edge(e));
    }
    let i = 0;
    if let Some(e_prev_i) = e_prev {
        let (a, b) = mesh.edges.split_at_mut(1);
        let mut e = &mut a[0];
        let mut prev = &mut b[e_prev_i - 1];
        set_adjacent_edges(vertices, e_prev_i, &mut prev, i, &mut e);
    }
    Ok(hull(mesh))
}

fn set_adjacent_edges(vertices: &Vec<Vertex2>, e_prev_i: usize, e_prev: &mut Edge, i: usize, e: &mut Edge) {
    if e_prev.cross(vertices, e) >= 0f32 {
        e_prev.edge_left_ccw = Some(i);
        e.edge_left_cw = Some(e_prev_i);
    } else {
        e_prev.edge_right_cw = Some(i);
        e.edge_left_ccw = Some(e_prev_i);
    }
}
