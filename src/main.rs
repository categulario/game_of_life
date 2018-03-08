extern crate clap;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

#[derive(Clone)]
enum CellType {
    Alive,
    Dead,
}

pub struct Game {
    gl: GlGraphics,
    data: Vec<CellType>,
    width: u32,
    height: u32,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let data = &self.data;
        let width = self.width;

        self.gl.draw(args.viewport(), |mut context, mut graphics| {
            clear([1.0; 4], graphics);

            for h in 0..(data.len() as u32)/width {
                for w in 0..width {
                    match data[(h*width + w) as usize] {
                        CellType::Alive => rectangle([0.0, 0.0, 0.0, 1.0], [(w*10) as f64, (h*10) as f64, 10.0, 10.0], context.transform, graphics),
                        _ => {},
                    }
                }
            }
        });
    }

    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    // Any live cell with two or three live neighbours lives on to the next generation.
    // Any live cell with more than three live neighbours dies, as if by overpopulation.
    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    fn update(&mut self, args: &UpdateArgs) {
        let mut newdata = Vec::new();
        {
            let data = &self.data;

            for cell in data {
                newdata.push(match cell {
                    &CellType::Alive => CellType::Dead,
                    &CellType::Dead => CellType::Alive,
                });
            }
        }

        self.data = newdata;
    }
}

fn data_from_file(filename:&str, width:u32, height:u32) -> Vec<CellType> {
    let f = File::open(filename).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut arr = vec![CellType::Dead; (width*height) as usize];

    for (h, line) in f.lines().enumerate() {
        if (h as u32) == height {
            break;
        }

        let line = line.expect("Something happened while reading a line from the file");

        for (w, c) in line.chars().enumerate() {
            if (w as u32) == width {
                break;
            }

            if c == 'x' {
                arr[h*(width as usize) + w] = CellType::Alive;
            }
        }
    }

    return arr;
}


fn main() {
    let matches = App::new("Game of life")
        .version("0.1.0")
        .author("Abraham Toriz <categulario@gmail.com>")
        .about("Runs a simulation of the game of life")
        .arg(Arg::with_name("FILE")
           .help("File that states initial status of the world")
           .required(true)
           .index(1))
        .arg(Arg::with_name("width")
           .short("w")
           .long("width")
           .value_name("WIDTH")
           .help("width of the grid")
           .default_value("64")
           .takes_value(true))
        .arg(Arg::with_name("height")
           .short("h")
           .long("height")
           .value_name("HEIGHT")
           .help("height of the grid")
           .default_value("48")
           .takes_value(true))
        .get_matches();

    let world_filename = matches.value_of("FILE").unwrap();

    let width:u32 = matches.value_of("width").unwrap().parse().expect("Need an integer for width");
    let height:u32 = matches.value_of("height").unwrap().parse().expect("Need an integer for height");

    let opengl = OpenGL::V3_3;

    let mut window:Window = WindowSettings::new(
            "Conway's Game of Life",
            [width*10, height*10]
        )
        .opengl(opengl)
        .srgb(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        data: data_from_file(world_filename, width, height),
        width: width,
        height: height,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
