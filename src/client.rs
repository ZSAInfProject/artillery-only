use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use enet::Enet;

use crate::network::Network;
use crate::structs::Map;

pub struct Client {
    gl: GlGraphics,
    map: Map,
    network: Network,
}

impl Client {
    fn render(&mut self, args: RenderArgs) {
        self.map.draw(&args, &mut self.gl);
    }

    pub fn new(enet_handle: Enet, opengl: OpenGL) -> Client {
        Client {
            gl: GlGraphics::new(opengl),
            map: Map::new(512, 512),
            network: Network::new(false, enet_handle).expect("Creating network for client failed"),
        }
    }
}

pub fn run_client(enet_handle: Enet) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Artillery only", (512, 512))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut client = Client::new(enet_handle, opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            client.render(r);
        }
    }
}
