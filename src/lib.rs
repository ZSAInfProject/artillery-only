#[macro_use]
extern crate serde_derive;
use std::env::Args;
use std::net::Ipv4Addr;
use std::process;

mod network;
pub mod structs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use self::network::message::*;
use self::network::Network;
use self::network::PeerData;
use self::structs::Map;

mod config;
pub use config::{ClientConfig, Config, ServerConfig};

struct Client {
    gl: GlGraphics,
    map: Map,
    network: Network,
}

impl Client {
    fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [72.0 / 255.0, 185.0 / 255.0, 219.0 / 255.0, 1.0];

        const GROUND: [f32; 4] = [127.0 / 255.0, 55.0 / 255.0, 14.0 / 255.0, 1.0];

        let ground: types::Rectangle = [0.0, 256.0, 512.0, 256.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLUE, gl);

            rectangle(GROUND, ground, c.transform, gl);
        });
    }
}

pub fn run_client(config: ClientConfig) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", (512, 512))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut client = Client {
        gl: GlGraphics::new(opengl),
        map: Map::new(512, 512),
        network: Network::new(false).expect("Creating network for client failed"),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            client.render(r);
        }
    }
}

struct Server {
    config: ServerConfig,
    map: Map,
    network: Network,
}

impl Server {
    fn lobby(&mut self) {
        let mut count = 0;
        while count < self.config.game_config.player_count {
            let mut msgs = self.network.update();
            while let Some((peer, msg)) = msgs.pop() {
                match msg {
                    Message::Initialize { nick } => count += 1,
                    Message::Disconnect => count -= 1,
                    _ => {}
                }
            }
        }
    }
    fn calculate(&mut self) {}
    fn send(&mut self) {}
    fn recive(&mut self) {}
}

pub fn run_server(server_config: ServerConfig) {
    let mut server = Server {
        map: Map::new(512, 512),
        network: Network::new(true).expect("Network for server not created"),
        config: server_config,
    };
    server.lobby();
    loop {
        server.send();
        server.recive();
        server.calculate();
    }
}

pub fn run(config: Config) {
    println!("{:#?}", config);

    let (client_config, server_config) = (config.client_config, config.server_config);

    if let Some(client_config) = client_config {
        run_client(client_config);
    }
    if let Some(server_config) = server_config {
        run_server(server_config);
    }
}
