use rand;
use rand::Rng;

use graphics::Renderer;

pub struct Network {
    agents: Vec<Agent>,
    rng: rand::ThreadRng,
}

impl Network {
    pub fn new() -> Network {
        Network {
            agents: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn lattice(n: usize) -> Network {
        let mut rng = rand::thread_rng();
        let mut network = Network::new();

        let side = (n as f64).sqrt() as i32;

        for i in 0..side {
            for j in 0..side {
                let p1 = get_rand(&mut rng, -0.2, 0.2);
                let p2 = get_rand(&mut rng, -0.2, 0.2);
                network.agents.push(Agent::new((i as f32 + p1, j as f32 + p2), 0.05, (0.0, 0.0, 0.0)));
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

    pub fn random(n: usize, p: f32) -> Network {
        let mut rng = rand::thread_rng();
        let mut network = Network::new();

        for _ in 0..n {
            network.add_agent();
        }

        for i in 0..n {
            for j in i+1..n {
                if get_rand(&mut rng, 0.0, 1.0) < p {
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
                                    0.05,
                                    (0.0, 0.0, 0.0)
                                    )
                         );
    }

    pub fn add_relation(&mut self, src: usize, dest: usize) {
        self.agents[src].relations.push(Relation::new(dest, (0.0, 0.0, 0.0)));
        self.agents[dest].relations.push(Relation::new(src, (0.0, 0.0, 0.0)));
    }

    pub fn smooth_till_rest(&mut self, dt: f32, thresh: f32, max: usize) {
        let mut total_vel = 2.0*thresh;

        let mut step = 0;
        while total_vel > thresh && step < max {
            self.smooth(dt);

            total_vel = 0.0;
            for i in self.agents.iter() {
                total_vel += i.vel.0.abs();
                total_vel += i.vel.1.abs();
            }

            step += 1;
        }
    }

    // Force driven smoothing using spring forces to keep 
    // adjacent vertices at a constant distance and coulomb 
    // force to keep non-adjacent vertices from clustering
    pub fn smooth(&mut self, dt: f32) {
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
        let cent = 0.2;

        for i in 0..self.agents.len() {
            let posi = self.agents[i].pos;

            // Spring force
            let mut f_spring = (0.0, 0.0);
            for j in self.agents[i].relations.iter() {
                let posj = self.agents[j.target].pos;

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
                let posj = self.agents[j].pos;

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
            f.0 -= d*self.agents[i].vel.0;
            f.1 -= d*self.agents[i].vel.1;

            // Centering force
            // -> to keep the vertices from floating away
            f.0 += -cent*posi.0;
            f.1 += -cent*posi.1;

            self.agents[i].vel.0 += dt*f.0;
            self.agents[i].vel.1 += dt*f.1;

            self.agents[i].pos.0 += dt*self.agents[i].vel.0;
            self.agents[i].pos.1 += dt*self.agents[i].vel.1;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);

        self.agents.iter().map(|ref a| renderer.draw_circle(a.pos, a.r, a.color)).count();
        for i in self.agents.iter() {
            for j in i.relations.iter() {
                renderer.draw_line(i.pos, self.agents[j.target].pos, j.color);
            }
        }

        renderer.end_frame();
    }
}

fn get_rand(rng: &mut rand::ThreadRng, a: f32, b: f32) -> f32 {
    (b - a) * rng.gen::<f32>() + a
}

pub struct Agent {
    pos: (f32, f32),
    vel: (f32, f32),

    r: f32,
    color: (f32, f32, f32),

    relations: Vec<Relation>,
}

impl Agent {
    fn new(pos: (f32, f32), r: f32, color: (f32, f32, f32)) -> Agent {
        Agent {
            pos: pos,
            vel: (0.0, 0.0),
            r: r,
            color: color,
            relations: Vec::new(),
        }
    }
}

pub struct Relation {
    target: usize,
    color: (f32, f32, f32),
}

impl Relation {
    fn new(target: usize, color: (f32, f32, f32)) -> Relation {
        Relation { target: target, color: color }
    }
}
