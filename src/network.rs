use std::cell::RefCell;

use rand;
use rand::Rng;

use graphics::Renderer;
use util::Vec2;

const AGENT_R: f32 = 0.25;

// Rest length of springs
const REST_LENGTH: f32 = 1.0;

// Damping factor
const DAMPING: f32 = 0.2;

// Stiffness of spring
const STIFFNESS: f32 = 1.0;

// Coulomb's constant
const COULOMB: f32 = 2.0;

// Distance lower bound for Coulomb
const DIST_BOUND: f32 = 0.1;

// Centering coefficient
const CENTERING: f32 = 0.05;

pub struct Network<T: AbstractComponent> {
    agents: Vec<Agent<T>>,
    rng: rand::ThreadRng,
}

impl<T: AbstractComponent> Network<T> {
    pub fn new() -> Network<T> {
        Network {
            agents: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn random(n: usize, p: f32) -> Network<T> {
        let mut network = Network::new();

        for _ in 0..n {
            network.add_agent();
        }

        for i in 0..n {
            for j in i+1..n {
                if get_rand(&mut network.rng, 0.0, 1.0) < p {
                    network.add_relation(i, j);
                }
            }
        }

        network
    }

    pub fn add_agent(&mut self) {
        self.agents.push(Agent::new(Vec2::new(get_rand(&mut self.rng, -10.0, 10.0), get_rand(&mut self.rng, -10.0, 10.0)),
                                    AGENT_R,
                                    (0.0, 0.0, 0.0),
                                    T::new(&mut self.rng)
                                    )
                         );
    }

    pub fn add_relation(&mut self, src: usize, dest: usize) {
        self.agents[src].relations.push(Relation::new(dest, (0.0, 0.0, 0.0)));
        self.agents[dest].relations.push(Relation::new(src, (0.0, 0.0, 0.0)));
    }

    pub fn physics_tick_till_rest(&mut self, dt: f32, thresh: f32, max: usize) {
        let mut total_vel = 2.0*thresh;

        let mut step = 0;
        while total_vel > thresh && step < max {
            self.physics_tick(dt);
            total_vel = self.total_vel();
            step += 1;
        }

    }

    fn total_vel(&self) -> f32 {
        let mut total_vel = 0.0;
        for i in self.agents.iter() {
            total_vel += i.physics.vel.abs().horizontal_sum();
        }
        total_vel
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

        for i in 0..self.agents.len() {
            let posi = self.agents[i].physics.pos;

            // Spring force
            let mut f_spring = Vec2::new(0.0, 0.0);
            for j in self.agents[i].relations.iter() {
                let posj = self.agents[j.target].physics.pos;

                let dir = posj - posi;
                let dist = dir.length();

                f_spring += dir.normalized().scale(k * dist - rest);
            }

            // Coulomb force
            let mut f_coulomb = Vec2::new(0.0, 0.0);
            for j in 0..self.agents.len() {
                if i == j {
                    continue;
                }
                let posj = self.agents[j].physics.pos;

                let dir = posj - posi;
                let dist = dir.length();

                let dist_bound = if dist < low {
                    low
                } else {
                    dist
                };

                f_coulomb += dir.normalized().scale(-k_e / (dist_bound * dist_bound));
            }

            let mut f = f_spring + f_coulomb;

            // Damping
            f -= self.agents[i].physics.vel.scale(d);

            // Centering force
            // -> to keep the vertices from floating away
            f += posi.scale(-cent);

            self.agents[i].physics.vel += f.scale(dt);

            self.agents[i].physics.pos += self.agents[i].physics.vel.scale(dt);
        }
    }

    pub fn logic_tick<F>(&mut self, set_logic: &F) where F: Fn(&RefCell<T>, Vec<&RefCell<T>>) {
        // TODO shuffle order

        for a in self.agents.iter() {
            set_logic(&a.logic, a.relations.iter().map(|r| &(self.agents[r.target].logic)).collect::<Vec<&RefCell<T>>>());
        }
    }

    pub fn set_appearance<F>(&mut self, set_appearance: &F) where F: Fn(&T, &mut PhysicsComponent) {
        for a in self.agents.iter_mut() {
            set_appearance(&a.logic.borrow(), &mut a.physics);
        }
    }

    pub fn debug<F>(&self, debug: &F) where F: Fn(Vec<&RefCell<T>>) {
        debug(self.agents.iter().map(|x| &x.logic).collect());
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);

        for i in self.agents.iter() {
            for j in i.relations.iter() {
                renderer.draw_line(i.physics.pos, self.agents[j.target].physics.pos, j.color);
            }
        }
        self.agents.iter().map(|ref a| renderer.draw_circle(a.physics.pos, a.physics.r, a.physics.color)).count();

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
    logic: RefCell<T>,

    relations: Vec<Relation>,
}

#[derive(Copy, Clone)]
pub struct Relation {
    target: usize,
    color: (f32, f32, f32),
}

impl Relation {
    fn new(target: usize, color: (f32, f32, f32)) -> Relation {
        Relation { target: target, color: color }
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

impl<T: AbstractComponent> Agent<T> {
    fn new(pos: Vec2, r: f32, color: (f32, f32, f32), ac: T) -> Agent<T> {
        let pc = PhysicsComponent::new(pos, Vec2::new(0.0, 0.0), r, color);

        Agent {
            physics: pc,
            logic: RefCell::new(ac),
            
            relations: Vec::new(),
        }
    }
}
