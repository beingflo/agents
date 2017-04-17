use graphics::Renderer;
use network::Network;
use network::AbstractComponent;
use input::InputHandler;

use input::InputEvent;
use rand;
use rand::Rng;

use util::Ticker;

const TIME_STEP: f32 = 0.05;
const VEL_THRESH: f32 = 0.001;
const IT_THRESH: usize = 2_000;

pub struct Simulation {
    renderer: Renderer,
    network: Network<LogicComponent>,
    input: InputHandler,

    n: usize,
    p: f32,

    freeze: bool,
}

impl Simulation {
    pub fn new(n: usize, p: f32) -> Simulation {
        let mut network = Network::random(n, p);
        network.physics_tick_till_rest(TIME_STEP, VEL_THRESH, IT_THRESH);

        let renderer = Renderer::new();
        let input = InputHandler::new();


        Simulation {
            n: n,
            p: p,
            renderer: renderer,
            network: network,
            input: input,
            freeze: true,
        }
    }

    pub fn run(&mut self) {
        let mut physics_ticker = Ticker::new(2);

        loop {
            self.network.draw(&mut self.renderer);

            if physics_ticker.tick() {
                self.network.physics_tick(TIME_STEP);
            }

            self.input.handle_events(self.renderer.display.poll_events());
            let events = self.input.get_events();
            if self.assign_events(events) {
                return;
            }
        }
    }

    fn assign_events(&mut self, events: Vec<InputEvent>) -> bool {
        for e in events.iter() {
            if let &InputEvent::ToggleFreeze = e {
                self.freeze = !self.freeze;
            }
            if let &InputEvent::Quit = e {
                return true;
            }
            if let &InputEvent::Rebuild = e {
                self.network = Network::random(self.n, self.p);
                self.network.physics_tick_till_rest(TIME_STEP, VEL_THRESH, IT_THRESH);
            }
        }

        self.renderer.apply_events(&events);
        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ProductionType {
    Hunter,
    Gatherer,
}

#[derive(Copy, Clone, Debug)]
struct LogicComponent {
    ptype: ProductionType,

    plant: u32,
    meat: u32,

    alive: bool,
    mark: bool,
}

impl AbstractComponent for LogicComponent {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        let p = rng.gen::<f32>();
        let ptype = if p < 0.5 {
            ProductionType::Hunter
        } else {
            ProductionType::Gatherer
        };

        LogicComponent { ptype: ptype, plant: 10, meat: 10, alive: true, mark: false }
    }
}
