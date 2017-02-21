#[macro_use]
extern crate glium;

mod graphics;
mod physics;

use graphics::Renderer;
use physics::{ Scene, Circle };

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    scene.add_circle(Circle::new((0.5, 0.5), 0.1));

    loop {
        scene.draw(&mut renderer);

        for e in renderer.display.poll_events() {
            use glium::glutin::Event;

            match e {
                Event::Closed => return,
                _ => false,
            };
        }
    }
}
