use graphics::Renderer;
use network::Network;


pub struct Simulation {
    renderer: Renderer,
    network: Network,
}

impl Simulation {
    pub fn new() -> Simulation {
        let renderer = Renderer::new();
        let network = Network::new();

        Simulation { renderer: renderer, network: network }
    }

    pub fn run(&mut self) {
        self.network.add_agent();
        self.network.add_agent();
        self.network.add_agent();

        self.network.add_relation(0, 1);
        self.network.add_relation(0, 2);
        self.network.add_relation(1, 2);

        let mut frame = 0;
        loop {
            frame += 1;
            println!("{}", frame);

            self.network.draw(&mut self.renderer);
            self.network.smooth(0.01);

            for e in self.renderer.display.poll_events() {
                use glium::glutin;
                use glium::glutin::Event;

                match e {
                    Event::Closed => return,
                    Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Q)) => return,
                    _ => (),
                };
            }
        }
    }
}
