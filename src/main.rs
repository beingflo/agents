#[macro_use]
extern crate glium;

mod graphics;
mod environment;

use graphics::Renderer;
use environment::{ Scene, Circle };

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    let handler = scene.add_circle(Circle::new((0.0, 0.0), 0.05));

    loop {
        scene.get_circle(&handler).shift((0.001, 0.0));

        scene.draw(&mut renderer);

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
