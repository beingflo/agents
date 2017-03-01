#[macro_use]
extern crate glium;
extern crate rand;

mod graphics;
mod environment;

use graphics::Renderer;
use environment::{ Scene, Circle, Line };

use rand::Rng;

fn main() {
    let mut renderer = Renderer::new();
    let mut scene = Scene::new(&renderer);

    let mut rng = rand::thread_rng();

    let mut circles = vec![];
    for i in 0..100 {
        let x = 2.0*rng.gen::<f32>()-1.0;
        let y = 2.0*rng.gen::<f32>()-1.0;
        let c = Circle::new((x, y), 0.02);
        circles.push(c);
        scene.add_circle(c);
    }

    for i in 0..circles.len() {
        for j in i..circles.len() {
            let p = rng.gen::<f32>();

            if p > 0.9 {
                scene.add_line(Line::new(circles[i].get_pos(), circles[j].get_pos()));
            }
        }
    }

    let mut frame = 0;
    loop {
        frame += 1;
        scene.draw(&mut renderer);
        println!("frame: {}", frame);

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
