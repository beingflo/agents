use glium::glutin;
use glium::backend::glutin_backend::PollEventsIter;
use glium::glutin::VirtualKeyCode as VK;

use std::collections::hash_set::HashSet;

pub struct InputHandler {
    keyset: HashSet<VK>,
    mouseset: [bool; 2],

    zoom: f32,

    mouse_pos: (i32, i32),
    mouse_pos_last_pressed: (i32, i32),

    mouse_scroll_sensitivity: f32,
    mouse_move_sensitivity: f32,
}

#[derive(Debug)]
pub enum Event {
    Quit,
    ToggleFreeze,
    Rebuild,
    Shift(f32, f32),
    Zoom(f32),
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            keyset: HashSet::new(),
            mouseset: [false; 2],
            zoom: 0.0,
            mouse_pos: (0, 0),
            mouse_pos_last_pressed: (0, 0),
            mouse_scroll_sensitivity: 0.2,
            mouse_move_sensitivity: 0.0026,
        }
    }

    pub fn handle_events(&mut self, events: PollEventsIter) {
        use glium::glutin::Event;

        for e in events {
            match e {
                Event::Closed => { self.keyset.insert(VK::Escape); },
                Event::MouseMoved(x, y) => self.mouse_moved_input(x, y),
                Event::MouseInput(state, button) => self.mouse_click_input(state, button),
                Event::MouseWheel(delta, _) => self.mouse_wheel_input(delta),
                Event::KeyboardInput(state, raw, code) => self.key_input(state, raw, code),
                _ => (),
            };
        }

    }

    fn key_input(&mut self, state: glutin::ElementState, raw: u8, code: Option<VK>) {
        use glium::glutin::ElementState as ES;

        match (state, raw, code) {
            (ES::Pressed, _, Some(virtcode)) => { self.keyset.insert(virtcode); },
            (ES::Released, _, Some(virtcode)) => { self.keyset.remove(&virtcode); },

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

        if self.keyset.contains(&VK::Q) || self.keyset.contains(&VK::Escape) {
            events.push(Event::Quit);
        }

        if self.keyset.contains(&VK::Space) {
            events.push(Event::ToggleFreeze);
            self.keyset.remove(&VK::Space);
        }

        if self.keyset.contains(&VK::R) {
            events.push(Event::Rebuild);
        }

        if self.mouseset[0] {
            let drag = (self.mouse_pos.0 - self.mouse_pos_last_pressed.0,
                        self.mouse_pos.1 - self.mouse_pos_last_pressed.1);
            let shift_x = drag.0 as f32 * self.mouse_move_sensitivity;
            let shift_y = drag.1 as f32 * self.mouse_move_sensitivity;
            events.push(Event::Shift(shift_x, -shift_y));
            self.mouse_pos_last_pressed = self.mouse_pos;
        }

        if self.zoom != 0.0 {
            events.push(Event::Zoom(self.zoom * self.mouse_scroll_sensitivity));
            self.zoom = 0.0;
        }

        events
    }
}
