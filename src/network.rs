use rand;
use rand::Rng;

use graphics::Renderer;

pub struct Network {
    agents: Vec<Agent>,
    relations: Vec<Relation>,
    rng: rand::ThreadRng,
}

impl Network {
    pub fn new() -> Network {
        Network { agents: Vec::new(), relations: Vec::new(), rng: rand::thread_rng() }
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
                    network.add_relation(AgentId(i), AgentId(j));
                }
            }
        }

        network
    }

    pub fn add_agent(&mut self) -> AgentId { 
        self.agents.push(Agent::new((get_rand(&mut self.rng, -1.0, 1.0),
                                     get_rand(&mut self.rng, -1.0, 1.0)),
                                    0.02,
                                    (0.0, 0.0, 0.0)));

        AgentId(self.agents.len()-1)
    }

    pub fn get_agent(&mut self, a: AgentId) -> &mut Agent {
        let AgentId(i) = a;
        &mut self.agents[i]
    }

    pub fn add_relation(&mut self, src: AgentId, dest: AgentId) -> RelationId {
        self.relations.push(Relation { source: src, target: dest, color: (0.0, 0.0, 0.0) });

        RelationId(self.relations.len()-1)
    }

    pub fn get_relation(&mut self, r: RelationId) -> &mut Relation {
        let RelationId(i) = r;
        &mut self.relations[i]
    }


    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.begin_frame();
        renderer.clear_color(1.0, 1.0, 1.0);

        self.agents.iter().map(|ref a| renderer.draw_circle(a.pos, a.r, a.color)).count();
        self.relations.iter().map(|ref r| renderer.draw_line(self.agents[r.source.0].pos, self.agents[r.target.0].pos, r.color)).count();

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
}

impl Agent {
    fn new(pos: (f32, f32), r: f32, color: (f32, f32, f32)) -> Agent {
        Agent { pos: pos, r: r, color: color }
    }

    fn set_color(&mut self, col: (f32, f32, f32)) {
        self.color = col;
    }

    fn set_r(&mut self, r: f32) {
        self.r = r;
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}

#[derive(Copy, Clone)]
pub struct AgentId(usize);

pub struct Relation {
    source: AgentId,
    target: AgentId,
    color: (f32, f32, f32),
}

#[derive(Copy, Clone)]
pub struct RelationId(usize);

impl Relation {
    fn set_color(&mut self, col: (f32, f32, f32)) {
        self.color = col;
    }
}


