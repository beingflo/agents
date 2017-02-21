use glium::{ DisplayBuild, glutin };
use glium::backend::glutin_backend::GlutinFacade;

pub struct Renderer {
    pub display: GlutinFacade,
}

impl Renderer {
    pub fn new() -> Renderer {
        let display = glutin::WindowBuilder::new().build_glium().unwrap();
        Renderer{ display: display }
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);
