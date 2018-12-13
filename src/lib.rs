extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate enet;

use std::{thread, time};

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

struct Client {
    gl: GlGraphics,
    map: Map,
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

pub fn run_client(ip: String, port: i32) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", (512, 512))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut client = Client {
        gl: GlGraphics::new(opengl),
        map: Map::new(512, 512),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            client.render(r);
        }
    }
}

struct Server {
    map: Map,
    network: Network,
}

impl Server {
    fn lobby(&mut self, player_count: i32) {
        let mut count = 0;
        while count < player_count {
            let mut msgs = self.network.update();
            while let Some((peer, msg)) = msgs.pop() {
                if let Message::Initialize { nick } = msg {
                    count += 1;
                }
            }
        }
    }
    fn calculate(&mut self) {}
    fn send(&mut self) {}
    fn recive(&mut self) {}
}

pub fn run_server(ip: String, port: i32, player_count: i32) {
    let mut server = Server {
        map: Map::new(512, 512),
        network: Network::new(true).expect("Network for server not created"),
    };
    server.lobby(player_count);
    loop {
        server.send();
        server.recive();
        server.calculate();
    }
}
