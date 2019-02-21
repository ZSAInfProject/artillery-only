#[macro_use]
extern crate serde_derive;

use std::thread::{self, JoinHandle};

mod network;
pub mod structs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use self::network::message::*;
use self::network::Network;
use self::structs::Map;

mod config;
pub use crate::config::{ClientConfig, Config, ServerConfig};

struct Client {
    gl: GlGraphics,
    map: Map,
    network: Network,
}

impl Client {
    fn render(&mut self, args: RenderArgs) {
        self.map.draw(&args, &mut self.gl);
    }
}

pub fn run_client() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Artillery only", (512, 512))
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
        while count < self.config.game.player_count {
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

    let (client, server) = (config.client, config.server);
    let mut client_thread: Option<JoinHandle<_>> = None;
    let mut server_thread: Option<JoinHandle<_>> = None;

    if let Some(_) = client {
        client_thread = Some(thread::spawn(move || run_client()));
    }
    if let Some(server_config) = server {
        server_thread = Some(thread::spawn(move || run_server(server_config)));
    }

    if let Some(client_thread) = client_thread.take() {
        client_thread.join().unwrap();
    }
    if let Some(server_thread) = server_thread.take() {
        server_thread.join().unwrap();
    }
}
