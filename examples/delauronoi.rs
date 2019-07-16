#[macro_use]
extern crate log;

use clap::App;
use env_logger;

fn main() {
    env_logger::init();

    let _app = App::new("delauronoi")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Delaunay triangulation of sets of points producing meshes and Voronoi diagrams")
        .get_matches();

    info!("Starting delauronoi");
}
