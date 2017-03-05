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
    let mut lines = vec![];

    let x1 = 2.0*rng.gen::<f32>()-1.0;
    let y1 = 2.0*rng.gen::<f32>()-1.0;
    let x2 = 2.0*rng.gen::<f32>()-1.0;
    let y2 = 2.0*rng.gen::<f32>()-1.0;

    circles.push((x1,y1));
    circles.push((x2,y2));

    lines.push(((x1,y1), (x2, y2)));

    let mut frame = 0;
    loop {
        frame += 1;
        //println!("{}", frame);
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);
        circles.iter().map(|&(x,y)| renderer.draw_circle((x, y), 0.02)).count();
        lines.iter().map(|&((x1,y1), (x2,y2))| renderer.draw_line((x1, y1), (x2,y2))).count();
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
