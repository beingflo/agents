#[macro_use]
extern crate glium;
extern crate rand;

mod graphics;
mod network;

use graphics::{ Renderer };

use rand::Rng;

fn main() {
    let mut renderer = Renderer::new();

    let mut rng = rand::thread_rng();

    let mut circles = vec![];

    for _ in 0..10 {
        let x = 2.0*rng.gen::<f32>()-1.0;
        let y = 2.0*rng.gen::<f32>()-1.0;

        circles.push((x,y));
    }

    loop {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);
        circles.iter().map(|&(x,y)| renderer.draw_circle((x, y), 0.02)).count();
        renderer.end_frame();

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
