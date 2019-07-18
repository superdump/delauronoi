#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

use clap::{App, Arg};
use delauronoi::*;
use env_logger;
use rand::Rng;

use coffee::graphics::{
    Color, Frame, Mesh, Shape, Window, WindowSettings,
};
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

    let width: usize = value_t!(app.value_of("width"), usize).unwrap_or(800);
    let height: usize = value_t!(app.value_of("height"), usize).unwrap_or(600);

    Delauronoi::run(WindowSettings {
        title: env!("CARGO_PKG_NAME").to_string(),
        size: (width as u32, height as u32),
        resizable: false,
        fullscreen: false,
    })
}

struct Delauronoi {
    vertices: Vertex2Array,
}

impl Delauronoi {
    const N_VERTICES: usize = 10;

    fn generate_vertices(width: f32, height: f32) -> Vertex2Array {
        let mut rng = rand::thread_rng();
        let mut vertices = v2_array_with_capacity(Self::N_VERTICES);
        for _ in 0..Self::N_VERTICES {
            vertices.push(v2(rng.gen_range(0f32, width), rng.gen_range(0f32, height)));
        }
        vertices
    }

    #[allow(dead_code)]
    fn print_vertices(vertices: &Vertex2Array) {
        for v in vertices {
            info!("{}", v);
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
            Delauronoi { vertices }
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        self.vertices.iter().for_each(|v| {
            let mut mesh = Mesh::new();
            let shape = Shape::Circle {
                center: *v,
                radius: 8f32,
            };
            mesh.fill(shape, Color::WHITE);
            mesh.draw(&mut frame.as_target());
        });
    }
}
