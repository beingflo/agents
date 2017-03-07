use glium::glutin;
use glium::backend::glutin_backend::PollEventsIter as PollEventsIter;

pub struct InputHandler {
    keyset: [bool; 5],
    mouseset: [bool; 2],
    zoom: f32,

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
        InputHandler { keyset: [false; 5], mouseset: [false; 2], zoom: 0.0, key_sensitivity: 0.01, mouse_sensitivity: 0.01 }
    }

    pub fn handle_events(&mut self, events: PollEventsIter) {
        use glium::glutin::Event;

        for e in events {
            match e {
                Event::Closed => self.keyset[0] = true,
                Event::MouseMoved(x,y) => self.mouse_moved_input(x,y),
                Event::MouseInput(state, button) => self.mouse_click_input(state, button),
                Event::MouseWheel(delta, _) => self.mouse_wheel_input(delta),
                Event::KeyboardInput(state, raw, code) => self.key_input(state, raw, code),
                _ => (),
            };
        }

    }

    fn key_input(&mut self, state: glutin::ElementState, raw: u8, code: Option<glutin::VirtualKeyCode>) {
        use glium::glutin::{ VirtualKeyCode };
        use glium::glutin::ElementState as ES;

        match (state, raw, code) {
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

    fn mouse_moved_input(&mut self, x: i32, y: i32) {
    }

    fn mouse_click_input(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        use glium::glutin::ElementState as ES;
        use glium::glutin::MouseButton as MB;

        match (state, button) {
            (ES::Pressed, MB::Left) => self.mouseset[0] = true,
            (ES::Released, MB::Left) => self.mouseset[0] = false,

            (ES::Pressed, MB::Right) => self.mouseset[1] = true,
            (ES::Released, MB::Right) => self.mouseset[1] = false,

            _ => (),
        };
    }

    fn mouse_wheel_input(&mut self, delta: glutin::MouseScrollDelta) {
        use glium::glutin::MouseScrollDelta as MSD;

        match delta {
            MSD::LineDelta(_, y) => self.zoom = self.mouse_sensitivity*y,
            MSD::PixelDelta(_, y) => self.zoom = self.mouse_sensitivity*y,
        }
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

        if self.mouseset[0] {

        }

        if self.mouseset[0] {

        }

        events.push(Event::Zoom(self.zoom));

        events
    }
}
