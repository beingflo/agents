use rand;
use rand::Rng;

use graphics::Renderer;

pub struct Network {
    agents: Vec<Agent>,
    rng: rand::ThreadRng,
}

impl Network {
    pub fn new() -> Network {
        Network { agents: Vec::new(), rng: rand::thread_rng() }
    }

    pub fn random(n: usize, p: f32) -> Network {
        let mut rng = rand::thread_rng();
        let mut network = Network::new();

        for _ in 0..n {
            network.add_agent();
        }

        for i in 0..n {
            for j in 0..n {
                if get_rand(&mut rng, 0.0, 1.0) < p {
                    network.add_relation(i, j);
                }
            }
        }

        network
    }

    pub fn add_agent(&mut self) { 
        self.agents.push(Agent::new((get_rand(&mut self.rng, -10.0, 10.0),
                                     get_rand(&mut self.rng, -10.0, 10.0)),
                                     0.05,
                                     (0.0, 0.0, 0.0)));
    }

    pub fn add_relation(&mut self, src: usize, dest: usize) {
        self.agents[src].relations.push(Relation { target: dest, color: (0.0, 0.0, 0.0) });
    }

    pub fn smooth(&mut self, dt: f32) {
        let rest = 5.0;

        for i in 0..self.agents.len() {
            let posi = self.agents[i].pos;

            let mut aggregate_force = (0.0, 0.0);

            for j in self.agents[i].relations.iter() {
                let posj = self.agents[j.target].pos;

                let dist = ((posj.0 - posi.0).powi(2) + (posj.1 - posi.1).powi(2)).sqrt();
                let dir = (posj.0 - posi.0, posj.1 - posi.1);

                aggregate_force.0 += (dist - rest) * dir.0;
                aggregate_force.1 += (dist - rest) * dir.1;
            }

            aggregate_force.0 *= dt;
            aggregate_force.1 *= dt;

            self.agents[i].pos = (posi.0 + aggregate_force.0, posi.1 + aggregate_force.1);
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
    (b-a) * rng.gen::<f32>() + a
}

pub struct Agent {
    pos: (f32, f32),
    r: f32,
    color: (f32, f32, f32),

    relations: Vec<Relation>,
}

impl Agent {
    fn new(pos: (f32, f32), r: f32, color: (f32, f32, f32)) -> Agent {
        Agent { pos: pos, r: r, color: color, relations: Vec::new() }
    }
}

pub struct Relation {
    target: usize,
    color: (f32, f32, f32),
}
