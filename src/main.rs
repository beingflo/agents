#[macro_use]
extern crate glium;
extern crate rand;
extern crate clap;

mod graphics;
mod network;
mod simulation;
mod graph;
mod input;
mod util;

use simulation::Simulation;
use util::get_args;

const NUM_AGENTS: usize = 100;
const DEGREE_P: f32 = 0.015;

fn main() {
    let (n, p) = get_args();

    let mut simulation = Simulation::new(n, p);

    simulation.run();
}
