use glium::glutin;
use glium::backend::glutin_backend::PollEventsIter as PollEventsIter;

pub struct InputHandler {
    keyset: [bool; 5],

    key_sensitivity: f32,
    mouse_sensitivity: f32,
}

pub enum Event {
    Quit,
    Shift(f32, f32),
    Zoom(f32),
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler { keyset: [false; 5], key_sensitivity: 0.01, mouse_sensitivity: 0.01 }
    }

    pub fn handle_events(&mut self, events: PollEventsIter) {
        use glium::glutin::Event;

        for e in events {
            match e {
                Event::Closed => self.keyset[0] = true,
                Event::KeyboardInput(status, raw, code) => self.key_input(status, raw, code),
                _ => (),
            };
        }

    }

    fn key_input(&mut self, status: glutin::ElementState, raw: u8, code: Option<glutin::VirtualKeyCode>) {
        use glium::glutin::{ VirtualKeyCode };
        use glium::glutin::ElementState as ES;

        match (status, raw, code) {
            (ES::Pressed, _, Some(VirtualKeyCode::Q)) =>        self.keyset[0] = true,
            (ES::Pressed, _, Some(VirtualKeyCode::Escape)) =>   self.keyset[0] = true,
            (ES::Pressed, _, Some(VirtualKeyCode::Right)) =>    self.keyset[1] = true,
            (ES::Released, _, Some(VirtualKeyCode::Right)) =>   self.keyset[1] = false,
            (ES::Pressed, _, Some(VirtualKeyCode::Left)) =>     self.keyset[2] = true,
            (ES::Released, _, Some(VirtualKeyCode::Left)) =>    self.keyset[2] = false,
            (ES::Pressed, _, Some(VirtualKeyCode::Up)) =>       self.keyset[3] = true,
            (ES::Released, _, Some(VirtualKeyCode::Up)) =>      self.keyset[3] = false,
            (ES::Pressed, _, Some(VirtualKeyCode::Down)) =>     self.keyset[4] = true,
            (ES::Released, _, Some(VirtualKeyCode::Down)) =>    self.keyset[4] = false,

            (_, _, _) => (),
        };
    }

    fn mouse_input() {

    }

    pub fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();

        if self.keyset[0] {
            events.push(Event::Quit);
        }

        if self.keyset[1] {
            events.push(Event::Shift(self.key_sensitivity, 0.0));
        }

        if self.keyset[2] {
            events.push(Event::Shift(-self.key_sensitivity, 0.0));
        }

        if self.keyset[3] {
            events.push(Event::Shift(0.0, self.key_sensitivity));
        }

        if self.keyset[4] {
            events.push(Event::Shift(0.0, -self.key_sensitivity));
        }

        events
    }
}
