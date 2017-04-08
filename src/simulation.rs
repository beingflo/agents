use graphics::Renderer;
use network::Network;
use network::AbstractComponent;
use network::PhysicsComponent;
use input::InputHandler;

use input::Event;
use rand;
use rand::Rng;

use std::cmp;
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
        let set_logic = |aref: &RefCell<LogicComponent>, bs: Vec<&RefCell<LogicComponent>>| {
            let mut a = aref.borrow_mut();

            if !a.alive {
                return;
            }

            // Death
            if a.meat <= 0 || a.plant <= 0 {
                a.alive = false;
                return;
            }

            // Consumption
            a.meat -= 1;
            a.plant -= 1;

            // Production
            if a.ptype == ProductionType::Hunter {
                a.meat += 2;
            } else {
                a.plant += 2;
            }

            for bref in bs.iter() {
                let mut b = bref.borrow_mut();

                if !b.alive {
                    continue;
                }

                if b.meat > a.meat && a.plant > b.plant {
                    let min = cmp::min(b.meat-a.meat, a.plant-b.plant);

                    b.meat -= min;
                    a.meat += min;

                    a.plant -= min;
                    b.plant += min;

                    break;
                }

                if a.meat > b.meat && b.plant > a.plant {
                    let min = cmp::min(a.meat-b.meat, b.plant-a.plant);

                    a.meat -= min;
                    b.meat += min;

                    b.plant -= min;
                    a.plant += min;

                    break;
                }
            }

        };

        let set_appearance = |a: &LogicComponent, b: &mut PhysicsComponent| {
            b.color = if a.ptype == ProductionType::Hunter {
                (0.5117, 0.168, 0.2422)
            } else {
                (0.0, 0.2422, 0.1836)
            };

            if !a.alive {
                b.color = (0.0, 0.0, 0.0);
            }
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
}

impl AbstractComponent for LogicComponent {
    fn new(rng: &mut rand::ThreadRng) -> Self {
        let p = rng.gen::<f32>();
        let ptype = if p > 0.5 {
            ProductionType::Hunter
        } else {
            ProductionType::Gatherer
        };

        LogicComponent { ptype: ptype, plant: 100, meat: 100, alive: true }
    }
}
