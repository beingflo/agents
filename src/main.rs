#[macro_use]
extern crate glium;
extern crate rand;

mod graphics;
mod network;
mod simulation;

use simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new();

    simulation.run();
}
