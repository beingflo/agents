use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use glium::{ self, DisplayBuild, glutin, Surface };
use glium::backend::glutin_backend::GlutinFacade;


pub struct Renderer {
    pub display: GlutinFacade,
    program: glium::Program,

    circle_mesh: CircleMesh,
    line_mesh: LineMesh,

    perspective: Option<[[f32; 4]; 4]>,
    frame: Option<glium::Frame>,
}

impl Renderer {
    pub fn new() -> Renderer {
        let display = glutin::WindowBuilder::new().build_glium().unwrap();
        let circle_mesh = CircleMesh::new(&display);
        let line_mesh = LineMesh::new(&display);
        let program = make_program(&display);

        Renderer{   display: display,
                    program: program,
                    circle_mesh: circle_mesh,
                    line_mesh: line_mesh,
                    perspective: None,
                    frame: None }
    }

    pub fn begin_frame(&mut self) {
        assert!(self.frame.is_none());

        self.frame = Some(self.display.draw());
        self.perspective = Some(get_perspective(&mut self.frame.as_mut().unwrap()));
    }

    pub fn clear_color(&mut self, r: f32, g: f32, b: f32) {
        assert!(self.frame.is_some());

        self.frame.as_mut().unwrap().clear_color(r, g, b, 0.0);
    }

    pub fn draw_circle(&mut self, pos: (f32, f32), r: f32, color: (f32, f32, f32)) {
        assert!(self.frame.is_some());

        let model = get_model_circle(pos, r);
        self.frame.as_mut().unwrap().draw(&self.circle_mesh.vertices, &self.circle_mesh.indices, &self.program,
                   &uniform!{ model: model, perspective: self.perspective.unwrap() }, &Default::default()).unwrap();
    }

    pub fn draw_line(&mut self, p1: (f32, f32), p2: (f32, f32), color: (f32, f32, f32)) {
        assert!(self.frame.is_some());

        let model = get_model_line(p1, p2);
        self.frame.as_mut().unwrap().draw(&self.line_mesh.vertices, &self.line_mesh.indices, &self.program,
                   &uniform!{ model: model, perspective: self.perspective.unwrap() }, &Default::default()).unwrap();
    }

    pub fn end_frame(&mut self) {
        assert!(self.frame.is_some());

        self.frame.take().unwrap().finish().unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);

struct CircleMesh {
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

const CIRCLE_RESOLUTION: i32 = 36;

impl CircleMesh {
    fn new(display: &GlutinFacade) -> Self {
        use std::f32;

        let segment_angle = 2.0*f32::consts::PI / CIRCLE_RESOLUTION as f32;

        let mut data = vec![];

        let mut cur_angle: f32 = 0.0;
        for _ in 0..CIRCLE_RESOLUTION {
            let p1 = Vertex { position: [0.0, 0.0] };
            let p2 = Vertex { position: [cur_angle.cos(), cur_angle.sin()] };
            let p3 = Vertex { position: [(cur_angle + segment_angle as f32).cos(), (cur_angle + segment_angle as f32).sin()] };

            data.push(p1);
            data.push(p2);
            data.push(p3);

            cur_angle += segment_angle;
        }

        let vertex_buffer = glium::VertexBuffer::new(display, &data).unwrap();
        CircleMesh { vertices: vertex_buffer, indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList) }
    }
}

struct LineMesh {
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl LineMesh {
    fn new(display: &GlutinFacade) -> Self {
        let p1 = Vertex { position: [0.0, 0.0] };
        let p2 = Vertex { position: [0.0, 1.0] };

        let data = vec![p1, p2];

        let vertex_buffer = glium::VertexBuffer::new(display, &data).unwrap();

        LineMesh { vertices: vertex_buffer, indices: glium::index::NoIndices(glium::index::PrimitiveType::LinesList) }
    }
}

fn get_perspective(frame: &glium::Frame) -> [[f32;4]; 4] {
    let perspective = {
        let (width, height) = frame.get_dimensions();
        let ar = height as f32 / width as f32;

        [
            [ar , 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    };

    perspective
}

fn get_model_circle(pos: (f32, f32), r: f32) ->  [[f32;4]; 4] {
    let model = {
        [
            [r, 0.0, 0.0, 0.0],
            [0.0, r, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [pos.0, pos.1, 0.0, 1.0],
        ]
    };

    model
}

fn get_model_line(p1: (f32, f32), p2: (f32, f32)) ->  [[f32;4]; 4] {
    let model = {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;

        [
            [0.0, 0.0, 0.0, 0.0],
            [dx, dy, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [p1.0, p1.1, 0.0, 1.0],
        ]
    };

    model
}

fn make_program(display: &GlutinFacade) -> glium::Program {
    let vertex_shader_src = load_shader("src/shader/vert_shader.glslv");
    let fragment_shader_src = load_shader("src/shader/frag_shader.glslf");

    glium::Program::from_source(display, &vertex_shader_src,
                                &fragment_shader_src, None).unwrap()
}

fn load_shader(file: &str) -> String {
    let path = Path::new(file);
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}
