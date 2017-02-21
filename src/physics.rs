use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use glium;
use glium::{ Surface };
use glium::backend::glutin_backend::GlutinFacade;

use graphics::{ Renderer, Vertex };

pub struct Scene {
    circles: (Vec<Circle>, Mesh),
    program: glium::Program,
}

impl Scene {
    pub fn new(renderer: &Renderer) -> Scene {
        Scene { circles: (vec![], Mesh::new(MeshKind::CircleMesh, &renderer.display)), program: make_program(&renderer.display) }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mut target = renderer.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        for c in &self.circles.0 {
            target.draw(&self.circles.1.vertices, &self.circles.1.indices, &self.program,
                        &uniform!{  pos_x: c.pos.0, pos_y: c.pos.1, r: c.r }, &Default::default()).unwrap();
        }

        target.finish().unwrap();
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.0.push(circle);
    }

    pub fn circle(&mut self, index: usize) -> Option<&mut Circle> {
        if index >= self.circles.0.len() {
            None
        } else {
            Some(&mut self.circles.0[index])
        }
    }

    pub fn remove_circle(&mut self, index: usize) {
        self.circles.0.remove(index);
    }
}

pub struct Circle {
    pos: (f32, f32),
    r: f32,
}

impl Circle {
    pub fn new(pos: (f32, f32), r: f32) -> Circle {
        Circle { pos: pos, r: r}
    }

    pub fn shift(&mut self, pos: (f32, f32)) {
        self.pos.0 += pos.0;
        self.pos.1 += pos.1;
    }

    pub fn grow(&mut self, r: f32) {
        self.r += r;
    }
}

enum MeshKind {
    CircleMesh,
}

struct Mesh {
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Mesh {
    fn new(kind: MeshKind, display: &GlutinFacade) -> Mesh {
        match kind {
            MeshKind::CircleMesh => {
                let p1 = Vertex { position: [-0.1, -0.1] };
                let p2 = Vertex { position: [-0.1, 0.1] };
                let p3 = Vertex { position: [0.1, -0.1] };
                let p4 = Vertex { position: [0.1, -0.1] };
                let p5 = Vertex { position: [0.1, 0.1] };
                let p6 = Vertex { position: [-0.1, 0.1] };
                let data = vec![p1, p2, p3, p4, p5, p6];

                let vertex_buffer = glium::VertexBuffer::new(display, &data).unwrap();
                Mesh { vertices: vertex_buffer, indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList) }
            }
        }

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
