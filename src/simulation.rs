use graphics::Renderer;
use network::Network;
use input::InputHandler;

use input::Event;

pub struct Simulation {
    renderer: Renderer,
    network: Network,
    input: InputHandler,
}

impl Simulation {
    pub fn new() -> Simulation {
        let renderer = Renderer::new();
        let network = Network::new();
        let input = InputHandler::new();

        Simulation { renderer: renderer, network: network, input: input }
    }

    pub fn run(&mut self) {
        let n = 20;
        for _ in 0..n {
            self.network.add_agent();
        }

        for i in 0..n {
            for k in 0..n {
                self.network.add_relation(i, k);
            }
        }

        loop {
            self.network.draw(&mut self.renderer);
            //self.network.smooth(0.1);

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
