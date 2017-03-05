use rand;
use rand::Rng;

use graphics::Renderer;

pub struct Network {
    agents: Vec<Agent>,
    n: usize,
    rng: rand::ThreadRng,
}

impl Network {
    pub fn new() -> Network {
        Network { agents: Vec::new(), n: 0, rng: rand::thread_rng() }
    }

    pub fn random(n: usize, p: f32) -> Network {
        let mut rng = rand::thread_rng();
        let mut network = Network::new();

        for i in 0..n {
            network.add_agent(); 
        }

        for i in 0..n {
            for j in i..n {
                if get_rand(&mut rng, 0.0, 1.0) < p {
                    network.add_edge(AgentId(i), AgentId(j));
                }
            }
        }

        network
    }

    pub fn add_agent(&mut self) -> AgentId { 
        self.agents.push(Agent::new(self.n, (get_rand(&mut self.rng, -1.0, 1.0),
                                             get_rand(&mut self.rng, -1.0, 1.0)),
                                            0.02,
                                            (0.0, 0.0, 0.0)));

        self.n += 1;
        AgentId(self.n - 1)
    }

    pub fn add_edge(&mut self, src: AgentId, dest: AgentId) {
        let AgentId(s) = src;

        self.agents[s].neighbors.push(dest);
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);

        self.agents.iter().map(|ref a| renderer.draw_circle(a.pos, a.r, a.color)).count();
        for i in 0..self.n {
            for j in self.agents[i].neighbors.iter() {
                let &AgentId(t) = j;
                renderer.draw_line(self.agents[i].pos, self.agents[t].pos, (0.0, 0.0, 0.0));
            }
        }

        renderer.end_frame();
    }

}

fn get_rand(rng: &mut rand::ThreadRng, a: f32, b: f32) -> f32 {
    (b-a) * rng.gen::<f32>() + a
}

struct Agent {
    id: AgentId,
    pos: (f32, f32),
    r: f32,
    color: (f32, f32, f32),

    neighbors: Vec<AgentId>,
}

impl Agent {
    fn new(id: usize, pos: (f32, f32), r: f32, color: (f32, f32, f32)) -> Agent {
        Agent { id: AgentId(id), pos: pos, r: r, color: color, neighbors: vec![] }
    }
}

#[derive(Copy, Clone)]
pub struct AgentId(usize);

