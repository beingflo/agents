use graphics::Renderer;
use network::Network;
use network::AbstractComponent;
use input::InputHandler;

use input::Event;

pub struct Simulation {
    renderer: Renderer,
    network: Network<MarketComponent>,
    input: InputHandler,
}

impl Simulation {
    pub fn new() -> Simulation {
        let mut network = Network::random(100, 0.02);
        //let mut network = Network::lattice(100);
        network.smooth_till_rest(0.05, 30.0, 1_000);

        let renderer = Renderer::new();
        let input = InputHandler::new();


        Simulation {
            renderer: renderer,
            network: network,
            input: input,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.network.draw(&mut self.renderer);
            self.network.smooth(0.05);

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

struct MarketComponent {
    ph: i32,
}

impl AbstractComponent for MarketComponent {
    fn new() -> Self {
        MarketComponent { ph: 0 }
    }
}


