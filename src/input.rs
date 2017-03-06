use glium::{ glutin };

pub struct KeyboardHandler {
    keyset: [bool; 8],
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        KeyboardHandler { keyset: [false; 8] }
    }

    pub fn keyinput(&mut self, status: glutin::ElementState, raw: u8, code: Option<glutin::VirtualKeyCode>) -> bool {
        use glium::glutin::{ VirtualKeyCode, ElementState };

        match (status, raw, code) {
            (ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => return true,
            (ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return true,
            (ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => self.keyset[0] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::Right)) => self.keyset[0] = false,
            (ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => self.keyset[1] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::Left)) => self.keyset[1] = false,
            (ElementState::Pressed, _, Some(VirtualKeyCode::Up)) => self.keyset[2] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::Up)) => self.keyset[2] = false,
            (ElementState::Pressed, _, Some(VirtualKeyCode::Down)) => self.keyset[3] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::Down)) => self.keyset[3] = false,

            (ElementState::Pressed, _, Some(VirtualKeyCode::W)) => self.keyset[4] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::W)) => self.keyset[4] = false,
            (ElementState::Pressed, _, Some(VirtualKeyCode::S)) => self.keyset[5] = true,
            (ElementState::Released, _, Some(VirtualKeyCode::S)) => self.keyset[5] = false,

            (ElementState::Pressed, _, Some(VirtualKeyCode::I)) => self.keyset[6] = true,
            (ElementState::Pressed, _, Some(VirtualKeyCode::J)) => self.keyset[7] = true,

            (_, _, _) => (),
        };

        false 
    }
}

pub struct MouseHandler {

}

impl MouseHandler {

}
