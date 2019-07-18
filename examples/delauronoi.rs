#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, Arg};
use delauronoi::*;
use env_logger;
use rand::Rng;

use coffee::graphics::{Color, Frame, Mesh, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    env_logger::init();

    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
        .get_matches();

    let width: usize = value_t!(app.value_of("width"), usize).unwrap_or(1080);
    let height: usize = value_t!(app.value_of("height"), usize).unwrap_or(1080);

    Delauronoi::run(WindowSettings {
        title: env!("CARGO_PKG_NAME").to_string(),
        size: (width as u32, height as u32),
        resizable: false,
        fullscreen: false,
    })
}

struct Delauronoi {
    vertices: Vec<Vertex2>,
    hull: Option<Hull>,
}

impl Delauronoi {
    const N_VERTICES: usize = 50;
    const BORDER: f32 = 20f32;

    fn generate_vertices(width: f32, height: f32) -> Vec<Vertex2> {
        let mut rng = rand::thread_rng();
        let mut vertices = Vec::<Vertex2>::with_capacity(Self::N_VERTICES);
        for _ in 0..Self::N_VERTICES {
            vertices.push(v2(
                rng.gen_range(Self::BORDER, width - Self::BORDER),
                rng.gen_range(Self::BORDER, height - Self::BORDER),
            ));
        }
        vertices
    }

    #[allow(dead_code)]
    fn print_vertices(vertices: &Vec<Vertex2>) {
        for v in vertices {
            info!("{}", v.position);
        }
    }
}

impl Game for Delauronoi {
    type Input = ();
    type LoadingScreen = ();

    fn load(window: &Window) -> Task<Delauronoi> {
        let (width, height) = (window.width(), window.height());
        Task::new(move || {
            let vertices = Self::generate_vertices(width, height);
            Self::print_vertices(&vertices);
            let hull = if let Ok(h) = quickhull(&vertices) {
                Some(h)
            } else {
                None
            };
            Delauronoi { vertices, hull }
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        // Draw circles for the vertices
        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let mut mesh = Mesh::new();
            let shape = Shape::Circle {
                center: v.position,
                radius: 8f32,
            };
            mesh.fill(shape, Color::WHITE);
            mesh.draw(&mut frame.as_target());
        }

        // Draw lines around the hull
        if let Some(hull) = &self.hull {
            let vertices = &self.vertices;
            hull.mesh.edges.iter().for_each(|e| {
                let mut mesh = Mesh::new();
                let points = line(
                    &vertices[e.origin].position,
                    &vertices[e.destination].position,
                    2f32,
                );
                let shape = Shape::Polyline { points };
                mesh.fill(shape, Color::WHITE);
                mesh.draw(&mut frame.as_target());
            });
        }
    }
}

fn line(a: &Position, b: &Position, width: f32) -> Vec<Position> {
    let mut v = Vec::new();
    let ab = (b - a).normalize() * width;
    let perp1 = Vector2::<f32>::new(ab.y, -ab.x);
    let perp2 = Vector2::<f32>::new(-ab.y, ab.x);
    v.push(position(a.x + perp1.x, a.y + perp1.y));
    v.push(position(b.x + perp1.x, b.y + perp1.y));
    v.push(position(b.x + perp2.x, b.y + perp2.y));
    v.push(position(a.x + perp2.x, a.y + perp2.y));
    v
}
