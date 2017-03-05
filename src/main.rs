#[macro_use]
extern crate glium;
extern crate rand;

mod graphics;
mod network;

use network::Network;
use graphics::Renderer;

fn main() {
    let mut renderer = Renderer::new();
    let mut network = Network::random(50, 0.2);

    let mut frame = 0;
    loop {
        frame += 1;
        println!("{}", frame);

        network.draw(&mut renderer);
        network.smooth(0.001);

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
