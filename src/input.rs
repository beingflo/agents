use glium::glutin;
use glium::backend::glutin_backend::PollEventsIter;

pub struct InputHandler {
    keyset: [bool; 5],
    mouseset: [bool; 2],
    space: bool,

    zoom: f32,

    mouse_pos: (i32, i32),
    mouse_pos_last_pressed: (i32, i32),

    key_sensitivity: f32,
    mouse_scroll_sensitivity: f32,
    mouse_move_sensitivity: f32,
}

pub enum Event {
    Quit,
    Start,
    Shift(f32, f32),
    Zoom(f32),
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            keyset: [false; 5],
            mouseset: [false; 2],
            space: false,
            zoom: 0.0,
            mouse_pos: (0, 0),
            mouse_pos_last_pressed: (0, 0),
            key_sensitivity: 0.01,
            mouse_scroll_sensitivity: 0.2,
            mouse_move_sensitivity: 0.0026,
        }
    }

    pub fn handle_events(&mut self, events: PollEventsIter) {
        use glium::glutin::Event;

        for e in events {
            match e {
                Event::Closed => self.keyset[0] = true,
                Event::MouseMoved(x, y) => self.mouse_moved_input(x, y),
                Event::MouseInput(state, button) => self.mouse_click_input(state, button),
                Event::MouseWheel(delta, _) => self.mouse_wheel_input(delta),
                Event::KeyboardInput(state, raw, code) => self.key_input(state, raw, code),
                _ => (),
            };
        }

    }

    fn key_input(&mut self,
                 state: glutin::ElementState,
                 raw: u8,
                 code: Option<glutin::VirtualKeyCode>) {
        use glium::glutin::VirtualKeyCode;
        use glium::glutin::ElementState as ES;

        match (state, raw, code) {
            (ES::Pressed, _, Some(VirtualKeyCode::Q)) => self.keyset[0] = true,
            (ES::Pressed, _, Some(VirtualKeyCode::Escape)) => self.keyset[0] = true,

            (ES::Pressed, _, Some(VirtualKeyCode::Right)) => self.keyset[1] = true,
            (ES::Released, _, Some(VirtualKeyCode::Right)) => self.keyset[1] = false,

            (ES::Pressed, _, Some(VirtualKeyCode::Left)) => self.keyset[2] = true,
            (ES::Released, _, Some(VirtualKeyCode::Left)) => self.keyset[2] = false,

            (ES::Pressed, _, Some(VirtualKeyCode::Up)) => self.keyset[3] = true,
            (ES::Released, _, Some(VirtualKeyCode::Up)) => self.keyset[3] = false,

            (ES::Pressed, _, Some(VirtualKeyCode::Down)) => self.keyset[4] = true,
            (ES::Released, _, Some(VirtualKeyCode::Down)) => self.keyset[4] = false,

            (ES::Pressed, _, Some(VirtualKeyCode::Space)) => self.space = !self.space,

            (_, _, _) => (),
        };
    }

    fn mouse_moved_input(&mut self, x: i32, y: i32) {
        self.mouse_pos = (x, y);
    }

    fn mouse_click_input(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        use glium::glutin::ElementState as ES;
        use glium::glutin::MouseButton as MB;

        match (state, button) {
            (ES::Pressed, MB::Left) => {
                self.mouseset[0] = true;
                self.mouse_pos_last_pressed = self.mouse_pos;
            }
            (ES::Released, MB::Left) => {
                self.mouseset[0] = false;
            }
            (ES::Pressed, MB::Right) => self.mouseset[1] = true,
            (ES::Released, MB::Right) => self.mouseset[1] = false,

            _ => (),
        };
    }

    fn mouse_wheel_input(&mut self, delta: glutin::MouseScrollDelta) {
        use glium::glutin::MouseScrollDelta as MSD;

        match delta {
            MSD::LineDelta(_, y) => self.zoom = y,
            MSD::PixelDelta(_, y) => self.zoom = y,
        }
    }

    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        if self.keyset[0] {
            events.push(Event::Quit);
        }

        if self.keyset[1] {
            events.push(Event::Shift(-self.key_sensitivity, 0.0));
        }

        if self.keyset[2] {
            events.push(Event::Shift(self.key_sensitivity, 0.0));
        }

        if self.keyset[3] {
            events.push(Event::Shift(0.0, -self.key_sensitivity));
        }

        if self.keyset[4] {
            events.push(Event::Shift(0.0, self.key_sensitivity));
        }

        if self.space {
            events.push(Event::Start);
            self.space = false;
        }

        if self.mouseset[0] {
            let drag = (self.mouse_pos.0 - self.mouse_pos_last_pressed.0,
                        self.mouse_pos.1 - self.mouse_pos_last_pressed.1);
            let shift_x = drag.0 as f32 * self.mouse_move_sensitivity;
            let shift_y = drag.1 as f32 * self.mouse_move_sensitivity;
            events.push(Event::Shift(shift_x, -shift_y));
            self.mouse_pos_last_pressed = self.mouse_pos;
        }

        events.push(Event::Zoom(self.zoom * self.mouse_scroll_sensitivity));
        self.zoom = 0.0;

        events
    }
}
