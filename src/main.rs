extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("Game of life")
        .version("0.1.0")
        .author("Abraham Toriz <categulario@gmail.com>")
        .about("Runs a simulation of the game of life")
        .arg(Arg::with_name("FILE")
           .help("File that states initial status of the world")
           .required(true)
           .index(1))
        .get_matches();

    let world_filename = matches.value_of("FILE").unwrap();

    let f = File::open(world_filename).expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Something happening while reading the line");
        println!("Line: {}", line);
    }
}
