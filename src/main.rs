#[macro_use]
extern crate glium;

mod graphics;
mod environment;

use graphics::Renderer;
use environment::{ Scene, Circle, Line };

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    let c1 = Circle::new((0.0, 0.0), 0.02);
    let c2 = Circle::new((0.4, 0.9), 0.02);

    let handler0 = scene.add_circle(c1);
    let handler1 = scene.add_circle(c2);

    let handler2 = scene.add_line(Line::new(c1.get_pos(), c2.get_pos()));

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
