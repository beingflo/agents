use graphics::Renderer;
use network::Network;
use network::AbstractComponent;
use network::PhysicsComponent;
use input::InputHandler;

use input::Event;
use rand;
use rand::Rng;

use ticker::Ticker;

use std::cmp;
use std::cell::RefCell;

const NUM_AGENTS: usize = 100;
const DEGREE_P: f32 = 0.015;

const TIME_STEP: f32 = 0.05;
const VEL_THRESH: f32 = 20.0;
const IT_THRESH: usize = 1_000;

pub struct Simulation {
    renderer: Renderer,
    network: Network<LogicComponent>,
    input: InputHandler,
    freeze: bool,
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut network = Network::random(NUM_AGENTS, DEGREE_P);
        network.physics_tick_till_rest(TIME_STEP, VEL_THRESH, IT_THRESH);

        let renderer = Renderer::new();
        let input = InputHandler::new();


        Simulation {
            renderer: renderer,
            network: network,
            input: input,
            freeze: true,
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
                    let min = (cmp::min(b.meat-a.meat, a.plant-b.plant) as f32 / 2.0) as u32;

                    b.meat -= min;
                    a.meat += min;

                    a.plant -= min;
                    b.plant += min;

                    break;
                }

                if a.meat > b.meat && b.plant > a.plant {
                    let min = (cmp::min(a.meat-b.meat, b.plant-a.plant) as f32 / 2.0) as u32;

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
                (0.9, 0.08, 0.04)
            } else {
                (0.0, 0.6, 0.1)
            };

            if a.mark {
                b.color = (0.0, 0.0, 1.0); 
            }

            if !a.alive {
                b.color = (0.0, 0.0, 0.0);
            }
        };

        let total = |xs: Vec<&RefCell<LogicComponent>>| {
            let mut sum = (0, 0);
            for bref in xs.iter() {
                let b = bref.borrow_mut();

                if !b.alive {
                    continue;
                }

                sum.0 += b.meat;
                sum.1 += b.plant;
            }
        };

        let max = |mut xs: Vec<&RefCell<LogicComponent>>| {
            let mut max = (0, 0);
            let mut index = 0;
            for (i, bref) in xs.iter().enumerate() {
                let b = bref.borrow_mut();

                if b.meat + b.plant > max.0 + max.1 && b.alive {
                    max.0 = b.meat;
                    max.1 = b.plant;
                    index = i;

                }
            }
            xs[index].borrow_mut().mark = true;
        };

        let mut logic_ticker = Ticker::new(10);
        let mut physics_ticker = Ticker::new(2);
        let mut debug_ticker = Ticker::new(10);
        let mut appearance_ticker = Ticker::new(10);

        loop {
            self.network.draw(&mut self.renderer);

            if physics_ticker.tick() {
                self.network.physics_tick(0.05);
            }

            if logic_ticker.tick() && !self.freeze {
                self.network.logic_tick(&set_logic);
            }

            if debug_ticker.tick() && !self.freeze {
                self.network.debug(&total);
                self.network.debug(&max);
            }

            if appearance_ticker.tick() {
                self.network.set_appearance(&set_appearance);
            }

            self.input.handle_events(self.renderer.display.poll_events());
            let events = self.input.get_events();
            if self.assign_events(events) {
                return;
            }
        }
    }

    fn assign_events(&mut self, events: Vec<Event>) -> bool {
        for e in events.iter() {
            if let &Event::ToggleFreeze = e {
                self.freeze = !self.freeze;
            }
            if let &Event::Quit = e {
                return true;
            }
            if let &Event::Rebuild = e {
                self.network = Network::random(NUM_AGENTS, DEGREE_P);
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
