use rand;
use rand::Rng;
use std::cell::RefCell;

use graphics::Renderer;

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

    pub fn lattice(n: usize) -> Network<T> {
        let mut network = Network::new();

        let side = (n as f64).sqrt() as i32;

        for i in 0..side {
            for j in 0..side {
                let p1 = get_rand(&mut network.rng, -0.2, 0.2);
                let p2 = get_rand(&mut network.rng, -0.2, 0.2);
                network.agents.push(Agent::new((i as f32 + p1, j as f32 + p2), 0.25, (0.0, 0.0, 0.0), T::new(&mut network.rng)));
            }
        }

        for i in 0..side*side {
            if i >= 1 && i % side != 0 {
                network.add_relation(i as usize, (i-1) as usize);
            }
            if i >= side {
                network.add_relation(i as usize, (i-side) as usize);
            }
        }

        network
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
        self.agents.push(Agent::new((   get_rand(&mut self.rng, -10.0, 10.0),
                                        get_rand(&mut self.rng, -10.0, 10.0)
                                    ),
                                    0.25,
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
            total_vel += i.physics.vel.0.abs();
            total_vel += i.physics.vel.1.abs();
        }
        total_vel
    }

    // Force driven smoothing using spring forces to keep 
    // adjacent vertices at a constant distance and coulomb 
    // force to keep non-adjacent vertices from clustering
    pub fn physics_tick(&mut self, dt: f32) {
        // Rest length of springs
        let rest = 1.0;

        // Damping factor
        let d = 0.2;

        // Stiffness of spring
        let k = 1.0;

        // Coulomb's constant
        let k_e = 2.0;

        // Distance lower bound for Coulomb
        let low = 0.1;

        // Centering coefficient
        let cent = 0.05;

        for i in 0..self.agents.len() {
            let posi = self.agents[i].physics.pos;

            // Spring force
            let mut f_spring = (0.0, 0.0);
            for j in self.agents[i].relations.iter() {
                let posj = self.agents[j.target].physics.pos;

                let dist = ((posj.0 - posi.0).powi(2) + (posj.1 - posi.1).powi(2)).sqrt();
                let dir = (posj.0 - posi.0, posj.1 - posi.1);

                f_spring.0 += k*(dist - rest) * (dir.0 / dist);
                f_spring.1 += k*(dist - rest) * (dir.1 / dist);
            }

            // Coulomb force
            let mut f_coulomb = (0.0, 0.0);
            for j in 0..self.agents.len() {
                if i == j {
                    continue;
                }
                let posj = self.agents[j].physics.pos;

                let dist = ((posj.0 - posi.0).powi(2) + (posj.1 - posi.1).powi(2)).sqrt();
                let dir = (posj.0 - posi.0, posj.1 - posi.1);

                let dist_bound = if dist < low {
                    low
                } else {
                    dist
                };

                f_coulomb.0 += -k_e/(dist_bound*dist_bound) * (dir.0 / dist_bound);
                f_coulomb.1 += -k_e/(dist_bound*dist_bound) * (dir.1 / dist_bound);
            }

            let mut f = (f_spring.0 + f_coulomb.0, f_spring.1 + f_coulomb.1);

            // Damping
            f.0 -= d*self.agents[i].physics.vel.0;
            f.1 -= d*self.agents[i].physics.vel.1;

            // Centering force
            // -> to keep the vertices from floating away
            f.0 += -cent*posi.0;
            f.1 += -cent*posi.1;

            self.agents[i].physics.vel.0 += dt*f.0;
            self.agents[i].physics.vel.1 += dt*f.1;

            self.agents[i].physics.pos.0 += dt*self.agents[i].physics.vel.0;
            self.agents[i].physics.pos.1 += dt*self.agents[i].physics.vel.1;
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
    pos: (f32, f32),
    vel: (f32, f32),
    pub r: f32,
    pub color: (f32, f32, f32),
}

impl PhysicsComponent {
    fn new(pos: (f32, f32), vel: (f32, f32), r: f32, color: (f32, f32, f32)) -> PhysicsComponent {
        PhysicsComponent {
            pos: pos,
            vel: vel,
            r: r,
            color: color,
        }
    }

}

impl<T: AbstractComponent> Agent<T> {
    fn new(pos: (f32, f32), r: f32, color: (f32, f32, f32), ac: T) -> Agent<T> {
        let pc = PhysicsComponent::new(pos, (0.0, 0.0), r, color);

        Agent {
            physics: pc,
            logic: RefCell::new(ac),
            
            relations: Vec::new(),
        }
    }
}
