#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, Arg};
use delauronoi::*;
use env_logger;
use rand::Rng;

fn main() {
    env_logger::init();

    let app = App::new("delauronoi")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Delaunay triangulation of sets of points producing meshes and Voronoi diagrams")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Window width")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .help("Window height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("points")
                .short("p")
                .long("points")
                .value_name("POINTS")
                .help("Number of random points to generate")
                .takes_value(true),
        )
        .get_matches();

    let n_vertices: usize = value_t!(app.value_of("points"), usize).unwrap_or(10);
    let width: usize = value_t!(app.value_of("width"), usize).unwrap_or(800);
    let height: usize = value_t!(app.value_of("height"), usize).unwrap_or(600);

    let vertices = generate_vertices(n_vertices, width, height);
    print_vertices(&vertices);
}

fn generate_vertices(n_vertices: usize, width: usize, height: usize) -> Vertex2Array {
    let mut rng = rand::thread_rng();
    let (w_f32, h_f32) = (width as f32, height as f32);
    let mut vertices = v2_array_with_capacity(n_vertices);
    for _ in 0..n_vertices {
        vertices.push(v2(rng.gen_range(0f32, w_f32), rng.gen_range(0f32, h_f32)));
    }
    vertices
}

fn print_vertices(vertices: &Vec<Point2<f32>>) {
    for v in vertices {
        info!("{}", v);
    }
}
