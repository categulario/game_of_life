use clap::{Arg, App};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use graphics::*;

#[derive(Clone)]
enum CellType {
    Alive,
    Dead,
}

fn alive_neighbours(data:&[CellType], index:usize, width:u32) -> u32 {
    let cur_row = ((index as u32)/width) as i32;
    let cur_col = ((index as u32)%width) as i32;
    let mut count = 0;

    for r in (cur_row-1)..(cur_row+2) {
        for c in (cur_col-1)..(cur_col+2) {
            let i = r*(width as i32) + c;

            if i>=0 && i < (data.len() as i32) && i != (index as i32) {
                count += match data[i as usize] {
                    CellType::Alive => 1,
                    _ => 0,
                }
            }
        }
    }

    return count;
}

pub struct Game {
    gl: GlGraphics,
    data: Vec<CellType>,
    width: u32,
    seconds: f64,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
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

    fn update(&mut self, args: &UpdateArgs, delay: f32) {
        self.seconds += args.dt;

        if self.seconds < delay as f64 {
            return;
        }

        let mut newdata = Vec::new();
        let data = &self.data;

        for (index, cell) in data.iter().enumerate() {
            newdata.push(match cell {
                &CellType::Alive => match alive_neighbours(data, index, self.width) {
                    0..=1 => CellType::Dead, // underpopulation
                    2..=3 => CellType::Alive, // normal population
                    4..=8 => CellType::Dead, // overpopulation
                    _ => CellType::Dead, // this doesn't really happen...
                },
                &CellType::Dead => match alive_neighbours(data, index, self.width) {
                    3 => CellType::Alive,
                    _ => CellType::Dead,
                },
            });
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
        .arg(Arg::with_name("delay")
             .short("d")
             .long("delay")
             .value_name("DELAY")
             .help("Milliseconds between steps")
             .default_value("0.5")
             .takes_value(true))
        .get_matches();

    let world_filename = matches.value_of("FILE").unwrap();

    let width:u32 = matches.value_of("width").unwrap().parse().expect("Need an integer for width");
    let height:u32 = matches.value_of("height").unwrap().parse().expect("Need an integer for height");
    let delay:f32 = matches.value_of("delay").unwrap().parse().expect("Need a float for delay");

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
        seconds: 0.0,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u, delay);
        }
    }
}
