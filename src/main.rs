extern crate clap;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use clap::{Arg, App};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

#[derive(Clone)]
enum CellType {
    Alive,
    Dead,
}

fn alive_neighbours(data:&[CellType], index:usize) -> u32 {
    return 1;
}

pub struct Game {
    gl: GlGraphics,
    data: Vec<CellType>,
    width: u32,
    height: u32,
    seconds: f64,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let data = &self.data;
        let width = self.width;

        self.gl.draw(args.viewport(), |context, graphics| {
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

    fn update(&mut self, args: &UpdateArgs) {
        self.seconds += args.dt;

        if self.seconds < 1.0 {
            return;
        }

        let mut newdata = Vec::new();

        {
            let data = &self.data;

            for (index, cell) in data.iter().enumerate() {
                newdata.push(match cell {
                    &CellType::Alive => match alive_neighbours(data, index) {
                        0...1 => CellType::Dead, // underpopulation
                        2...3 => CellType::Alive, // normal population
                        3...8 => CellType::Dead, // overpopulation
                        _ => CellType::Dead, // this doesn't really happen...
                    },
                    &CellType::Dead => match alive_neighbours(data, index) {
                        3 => CellType::Alive,
                        _ => CellType::Dead,
                    },
                });
            }
        }

        self.data = newdata;
        self.seconds = 0.0;
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
        seconds: 0.0,
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
