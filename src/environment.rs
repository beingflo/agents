use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use glium;
use glium::{ Surface };
use glium::backend::glutin_backend::GlutinFacade;

use graphics::{ Renderer, Vertex };

pub struct Scene {
    circles: Vec<Circle>,
    lines: Vec<Line>,

    program: glium::Program,
    circle_mesh: CircleMesh,
    line_mesh: LineMesh,
}

struct ObjectHandle(usize);

impl Scene {
    pub fn new(renderer: &Renderer) -> Scene {
        Scene { circles: vec![],
                lines: vec![],
                circle_mesh: CircleMesh::new(&renderer.display),
                line_mesh: LineMesh::new(&renderer.display),
                program: make_program(&renderer.display) }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mut target = renderer.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        for c in &self.circles {
            c.draw(&self.circle_mesh, &self.program, &mut target);
        }

        for l in &self.lines {
            l.draw(&self.line_mesh, &self.program, &mut target);
        }

        target.finish().unwrap();
    }

    pub fn add_circle(&mut self, c: Circle) -> ObjectHandle {
        self.circles.push(c);
        ObjectHandle(self.circles.len() - 1)
    }

    pub fn add_line(&mut self, l: Line) -> ObjectHandle {
        self.lines.push(l);
        ObjectHandle(self.lines.len() - 1)
    }

    pub fn get_circle(&mut self, handle: &ObjectHandle) -> &mut Circle {
        let index = handle.0;
        assert!(index < self.circles.len());

        &mut self.circles[index]
    }

    pub fn get_line(&mut self, handle: &ObjectHandle) -> &mut Line {
        let index = handle.0;
        assert!(index < self.lines.len());

        &mut self.lines[index]
    }
}

pub struct Circle {
    pos: (f32, f32),
    r: f32,
}

impl Circle {
    pub fn new(pos: (f32, f32), r: f32) -> Self {
        Circle { pos: pos, r: r }
    }

    pub fn shift(&mut self, pos: (f32, f32)) {
        self.pos.0 += pos.0;
        self.pos.1 += pos.1;
    }

    pub fn grow(&mut self, r: f32) {
        self.r += r;
    }

    fn draw(&self, mesh: &CircleMesh, program: &glium::Program, frame: &mut glium::Frame) {
        frame.draw(&mesh.vertices, &mesh.indices, program,
                   &uniform!{  pos_x: self.pos.0, pos_y: self.pos.1, r: self.r }, &Default::default()).unwrap();
    }

}

pub struct Line {
    p1: (f32, f32),
    p2: (f32, f32),
}

impl Line {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Line { p1: p1, p2: p2 }
    }

    pub fn set_p1(&mut self, p: (f32, f32)) {
        self.p1.0 = p.0;
        self.p1.1 = p.1;
    }

    pub fn set_p2(&mut self, p: (f32, f32)) {
        self.p2.0 = p.0;
        self.p2.1 = p.1;
    }

    fn draw(&self, mesh: &LineMesh, program: &glium::Program, frame: &mut glium::Frame) {
        frame.draw(&mesh.vertices, &mesh.indices, program,
                   &uniform!{  pos_x: self.p1.0, pos_y: self.p1.1, r: self.p2.0 }, &Default::default()).unwrap();
    }
}

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
