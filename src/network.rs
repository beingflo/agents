use std::cell::RefCell;

use rand;
use rand::Rng;

use graphics::Renderer;
use util::Vec2;

use graph::{ Graph, Node, Edge, NodeIndex };

const AGENT_R: f32 = 0.25;

// Rest length of springs
const REST_LENGTH: f32 = 1.0;

// Damping factor
const DAMPING: f32 = 0.7;

// Stiffness of spring
const STIFFNESS: f32 = 1.0;

// Coulomb's constant
const COULOMB: f32 = 2.0;

// Distance lower bound for Coulomb
const DIST_BOUND: f32 = 0.1;

// Centering coefficient
const CENTERING: f32 = 0.05;

pub struct Network<T: AbstractComponent> {
    graph: Graph<Agent<T>, Relation>,
    rng: rand::ThreadRng,
}

impl<T: AbstractComponent> Network<T> {
    pub fn new() -> Network<T> {
        Network {
            graph: Graph::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn random(n: usize, p: f32) -> Network<T> {
        let mut network = Network::new();

        let mut nodes = Vec::new();
        for _ in 0..n {
            nodes.push(network.add_agent());
        }

        for i in 0..n {
            for j in i+1..n {
                if get_rand(&mut network.rng, 0.0, 1.0) < p {
                    network.add_relation(nodes[i], nodes[j]);
                }
            }
        }

        network
    }

    pub fn add_agent(&mut self) -> NodeIndex{
        self.graph.add_node(Agent::new(Vec2::new(get_rand(&mut self.rng, -20.0, 20.0), get_rand(&mut self.rng, -20.0, 20.0)),
                                    AGENT_R,
                                    (0.0, 0.0, 0.0),
                                    T::new(&mut self.rng)
                                    )
                         )
    }

    pub fn add_relation(&mut self, src: NodeIndex, dest: NodeIndex) {
        self.graph.add_edge(src, dest, Relation::new((0.0, 0.0, 0.0)));
    }

    pub fn physics_tick_till_rest(&mut self, dt: f32, thresh: f32, max: usize) {
        let mut avg_vel = 2.0*thresh;

        let mut step = 0;
        while avg_vel > thresh && step < max {
            self.physics_tick(dt);
            avg_vel = self.average_vel();
            step += 1;
        }

    }

    fn average_vel(&self) -> f32 {
        let mut total_vel = 0.0;
        //for i in self.agents.iter() {
        //    total_vel += i.physics.vel.abs().horizontal_sum();
        //}
        //total_vel / self.agents.len() as f32
        0.0
    }

    // Force driven smoothing using spring forces to keep 
    // adjacent vertices at a constant distance and coulomb 
    // force to keep non-adjacent vertices from clustering
    pub fn physics_tick(&mut self, dt: f32) {
        let rest = REST_LENGTH;
        let d = DAMPING;
        let k = STIFFNESS;
        let k_e = COULOMB;
        let low = DIST_BOUND;
        let cent = CENTERING;

        //for i in 0..self.agents.len() {
        //    let posi = self.agents[i].physics.pos;

        //    // Spring force
        //    let mut f_spring = Vec2::new(0.0, 0.0);
        //    for j in self.agents[i].relations.iter() {
        //        let posj = self.agents[j.target].physics.pos;

        //        let dir = posj - posi;
        //        let dist = dir.length();

        //        f_spring += dir.normalized().scale(k * (dist - rest));
        //    }

        //    // Coulomb force
        //    let mut f_coulomb = Vec2::new(0.0, 0.0);
        //    for j in 0..self.agents.len() {
        //        if i == j {
        //            continue;
        //        }
        //        let posj = self.agents[j].physics.pos;

        //        let dir = posj - posi;
        //        let dist = dir.length();

        //        let dist_bound = if dist < low {
        //            low
        //        } else {
        //            dist
        //        };

        //        f_coulomb += dir.normalized().scale(-k_e / (dist_bound * dist_bound));
        //    }

        //    let mut f = f_spring + f_coulomb;

        //    // Damping
        //    f -= self.agents[i].physics.vel.scale(d);

        //    // Centering force
        //    // -> to keep the vertices from floating away
        //    f += posi.scale(-cent);

        //    self.agents[i].physics.vel += f.scale(dt);

        //    self.agents[i].physics.pos += self.agents[i].physics.vel.scale(dt);
        //}
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);

        //for i in self.agents.iter() {
        //    for j in i.relations.iter() {
        //        renderer.draw_line(i.physics.pos, self.agents[j.target].physics.pos, j.color);
        //    }
        //}
        //self.agents.iter().map(|ref a| renderer.draw_circle(a.physics.pos, a.physics.r, a.physics.color)).count();

        renderer.end_frame();
    }
}

fn get_rand(rng: &mut rand::ThreadRng, a: f32, b: f32) -> f32 {
    (b - a) * rng.gen::<f32>() + a
}

pub trait AbstractComponent : Copy {
    fn new(&mut rand::ThreadRng) -> Self;
}

#[derive(Clone)]
pub struct Agent<T: AbstractComponent> {
    physics: PhysicsComponent,
    logic: T,
}

impl<T: AbstractComponent> Agent<T> {
    fn new(pos: Vec2, r: f32, color: (f32, f32, f32), ac: T) -> Agent<T> {
        let pc = PhysicsComponent::new(pos, Vec2::new(0.0, 0.0), r, color);

        Agent {
            physics: pc,
            logic: ac,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Relation {
    color: (f32, f32, f32),
}

impl Relation {
    fn new(color: (f32, f32, f32)) -> Relation {
        Relation { color: color }
    }
}

#[derive(Copy, Clone)]
pub struct PhysicsComponent {
    pos: Vec2,
    vel: Vec2,
    pub r: f32,
    pub color: (f32, f32, f32),
}

impl PhysicsComponent {
    fn new(pos: Vec2, vel: Vec2, r: f32, color: (f32, f32, f32)) -> PhysicsComponent {
        PhysicsComponent {
            pos: pos,
            vel: vel,
            r: r,
            color: color,
        }
    }

}
