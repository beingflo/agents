#[macro_use]
extern crate glium;

mod graphics;
mod physics;

use graphics::Renderer;
use physics::{ Scene, Circle };

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    scene.add_circle(Circle::new((0.0, 0.0), 0.1));
    scene.add_circle(Circle::new((0.5, 0.0), 0.1));

    loop {
        scene.draw(&mut renderer);

        scene.circle(0).unwrap().shift((-0.001,0.0));
        scene.circle(1).unwrap().shift((0.0, -0.001));

        for e in renderer.display.poll_events() {
            use glium::glutin::Event;

            match e {
                Event::Closed => return,
                _ => false,
            };
        }
    }
}
