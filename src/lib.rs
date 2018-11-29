pub mod structs;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::collections::HashMap;

use self::structs::Map;

struct App {
    gl: GlGraphics,
    map: Map,
    is_my_turn: bool,
    angle: f32,
    keys: HashMap<piston::input::Key, bool>,
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

    fn get_key(&mut self, key: piston::input::Key) -> bool {
        *self.keys.entry(key).or_insert(false)
    }

    fn update(&mut self, args: UpdateArgs) {
        println!("{:}", self.get_key(piston::input::Key::Left));
    }

    fn press(&mut self, button: piston::input::Button) {
        if let piston::input::Button::Keyboard(b) = button {
            self.keys.insert(b, true);
        }
    }

    fn release(&mut self, button: piston::input::Button) {
        if let piston::input::Button::Keyboard(b) = button {
            self.keys.insert(b, false);
        }
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
        is_my_turn: false,
        angle: 0f32,
        keys: HashMap::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        e.render(|args| app.render(*args));

        e.update(|args| app.update(*args));

        e.press(|button| app.press(button));
        e.release(|button| app.release(button));
    }
}
