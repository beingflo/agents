#[macro_use]
extern crate glium;

mod graphics;
mod environment;

use graphics::Renderer;
use environment::{ Scene, Circle, Line };

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    let handler0 = scene.add_circle(Circle::new((0.0, 0.0), 0.05));
    let handler1 = scene.add_circle(Circle::new((-0.4, 0.9), 0.05));

    let handler2 = scene.add_line(Line::new((0.0,0.0),(-0.4,0.9)));

    loop {

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
