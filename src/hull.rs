use crate::mesh::Mesh;

use std::collections::HashSet;

pub struct Hull {
    pub mesh: Mesh,
    is_convex: Option<bool>,
}

pub fn hull(mesh: Mesh) -> Hull {
    Hull {
        mesh,
        is_convex: None,
    }
}

impl Hull {
    pub fn is_convex(&mut self) -> bool {
        if let Some(is_convex) = self.is_convex {
            return is_convex;
        }
        let edges = &self.mesh.edges;
        let mut is_cw_opt: Option<bool> = None;
        let mut i = 0;
        let mut seen = HashSet::new();
        loop {
            if seen.contains(&i) {
                break;
            }
            seen.insert(i);
            let e = &edges[i];
            if (e.edge_left_ccw.is_some() && e.edge_right_cw.is_some())
                || (e.edge_left_cw.is_some() && e.edge_right_ccw.is_some())
            {
                self.is_convex = Some(false);
                return false;
            }
            if is_cw_opt.is_none() {
                is_cw_opt = Some(e.edge_right_cw.is_some());
            }
            if is_cw_opt.unwrap() {
                if let Some(next) = e.edge_right_cw {
                    i = next;
                } else {
                    self.is_convex = Some(false);
                    return false;
                }
            } else {
                if let Some(next) = e.edge_left_ccw {
                    i = next;
                } else {
                    self.is_convex = Some(false);
                    return false;
                }
            }
        }
        self.is_convex = Some(seen.len() == edges.len());
        return true;
    }
}
