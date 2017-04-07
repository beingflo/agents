use graphics::Renderer;
use network::Network;
use network::AbstractComponent;
use network::PhysicsComponent;
use input::InputHandler;

use input::Event;
use rand;
use rand::Rng;

use std::cell::RefCell;

pub struct Simulation {
    renderer: Renderer,
    network: Network<LogicComponent>,
    input: InputHandler,
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut network = Network::random(100, 0.02);
        network.physics_tick_till_rest(0.05, 30.0, 1_000);

        let renderer = Renderer::new();
        let input = InputHandler::new();


        Simulation {
            renderer: renderer,
            network: network,
            input: input,
        }
    }

    pub fn run(&mut self) {
        let set_logic = |a: &RefCell<LogicComponent>, bs: Vec<&RefCell<LogicComponent>>| {
            let mut ab = a.borrow_mut();
            if ab.ptype == ProductionType::Hunter {
                ab.color.0 += 0.002;
            } else {
                ab.color.1 += 0.002;
            }
        };

        let set_appearance = |a: &LogicComponent, b: &mut PhysicsComponent| {
            b.color = a.color;
        };

        loop {
            self.network.draw(&mut self.renderer);

            self.network.physics_tick(0.05);
            self.network.logic_tick(&set_logic);

            self.network.set_appearance(&set_appearance);

            self.input.handle_events(self.renderer.display.poll_events());
            let events = self.input.get_events();
            if self.assign_events(events) {
                return;
            }
        }
    }

    fn assign_events(&mut self, events: Vec<Event>) -> bool {
        for e in events.iter() {
            if let &Event::Quit = e {
                return true;
            }
        }

        self.renderer.apply_events(&events);
        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum ProductionType {
    Hunter,
    Gatherer,
}

#[derive(Copy, Clone)]
struct LogicComponent {
    ptype: ProductionType,

    color: (f32, f32, f32),
    plant_amount: u32,
    meat_amount: u32,
}

impl AbstractComponent for LogicComponent {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        let p = rng.gen::<f32>();
        let ptype = if p > 0.5 {
            ProductionType::Hunter
        } else {
            ProductionType::Gatherer
        };

        LogicComponent { ptype: ptype, plant_amount: 100, meat_amount: 100, color: (0.0, 0.0, 0.0) }
    }
}
