use clap::App;

fn main() {
    let app = App::new("Delauronoi")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Delaunay triangulation of sets of points producing meshes and Voronoi diagrams")
        .get_matches();
}
