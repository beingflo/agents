#[macro_use]
extern crate glium;
extern crate rand;

mod graphics;
mod network;

use network::Network;
use graphics::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    let mut network = Network::new();

    for _ in 0..20 {
        network.add_agent();
    }

    for i in 0..20 {
        let mut min1: f32 = 10.0;
        let mut min1i = 0;
        let mut min2: f32 = 10.0;
        let mut min2i = 0;
        let mut min3: f32 = 10.0;
        let mut min3i = 0;
        let mut min4: f32 = 10.0;
        let mut min4i = 0;

        let ai = network.get_agent(i).unwrap().pos;
        for j in 0..20 {
            let aj = network.get_agent(j).unwrap().pos;
            let tmp = ((ai.0 - aj.0).powi(2) + (ai.1 - aj.1).powi(2)).sqrt();

            if tmp < min1 {
                min1 = tmp;
                min1i = j;
            }
        }

        for j in 0..20 {
            let aj = network.get_agent(j).unwrap().pos;
            let tmp = ((ai.0 - aj.0).powi(2) + (ai.1 - aj.1).powi(2)).sqrt();

            if j == min1i {
                continue;
            }

            if tmp < min2 {
                min2 = tmp;
                min2i = j;
            }
        }

        for j in 0..20 {
            let aj = network.get_agent(j).unwrap().pos;
            let tmp = ((ai.0 - aj.0).powi(2) + (ai.1 - aj.1).powi(2)).sqrt();

            if j == min1i || j == min2i {
                continue;
            }

            if tmp < min3 {
                min3 = tmp;
                min3i = j;
            }
        }

        for j in 0..20 {
            let aj = network.get_agent(j).unwrap().pos;
            let tmp = ((ai.0 - aj.0).powi(2) + (ai.1 - aj.1).powi(2)).sqrt();

            if j == min1i || j == min2i || j == min3i {
                continue;
            }

            if tmp < min4 {
                min4 = tmp;
                min4i = j;
            }
        }

        network.add_relation(i, min1i);
        network.add_relation(i, min2i);
        network.add_relation(i, min3i);
        network.add_relation(i, min4i);
    }

    let mut frame = 0;
    loop {
        frame += 1;
        println!("{}", frame);

        network.draw(&mut renderer);
        network.smooth(0.02);

        for e in renderer.display.poll_events() {
            use glium::glutin;
            use glium::glutin::Event;

            match e {
                Event::Closed => return,
                Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Q)) => return,
                _ => (),
            };
        }
    }
}
