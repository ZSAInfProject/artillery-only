extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate enet;
extern crate bincode;

use std::{thread, time};

mod network;
pub mod structs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use self::structs::Map;
use self::network::PeerData;
use self::network::message::*;

struct App {
    gl: GlGraphics,
    map: Map,
}

impl App {
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

pub fn run_client() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", (512, 512))
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        map: Map::new(512, 512),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(r);
        }
    }
}