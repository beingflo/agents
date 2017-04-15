#[macro_use]
extern crate glium;
extern crate rand;
extern crate clap;

mod graphics;
mod network;
mod simulation;
mod input;
mod util;

use simulation::Simulation;
use clap::{Arg, App};

const NUM_AGENTS: usize = 100;
const DEGREE_P: f32 = 0.015;

fn main() {
    let (n, p) = get_args();
    let mut simulation = Simulation::new(n, p);

    simulation.run();
}

fn get_args() -> (usize, f32) {
    let matches = App::new("Agent based simulation")
        .version("0.1.0")
        .author("Florian Marending")
        .arg(Arg::with_name("n")
             .short("n")
             .long("num_agents")
             .help("Sets the number of agents in the simulation")
             .takes_value(true))
        .arg(Arg::with_name("d")
             .short("d")
             .long("neighbor_degree")
             .help("Sets the neighborhood degree in the network")
             .takes_value(true))
        .get_matches();

    let mut n = NUM_AGENTS;
    let mut p = DEGREE_P;

    if let Some(x) = matches.value_of("n") {
        n = x.parse::<usize>().expect("Error: Not a number");
    }

    if let Some(x) = matches.value_of("d") {
        p = x.parse::<f32>().expect("Error: Not a number");
    }

    (n, p)
}
