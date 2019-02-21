use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    terrain: Vec<bool>,
    tanks: HashMap<u32, Tank>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut terrain: Vec<bool> = Vec::new();
        for _ in 0..(width * height) {
            terrain.push(false);
        }
        for _ in 0..(width * height / 2) {
            terrain.push(true);
        }
        Map {
            width,
            height,
            terrain,
            tanks: HashMap::new(),
        }
    }

    fn get_tile(self, x: usize, y: usize) -> Option<bool> {
        let value = self.terrain.get(y * self.width + x);
        if let Some(val) = value {
            return Some(*val);
        }
        return None;
    }
    fn set_tile(mut self, is_solid: bool, x: usize, y: usize) {
        self.terrain[y * self.width + x] = is_solid;
    }

    pub fn draw(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        const BLUE: [f32; 4] = [72.0 / 255.0, 185.0 / 255.0, 219.0 / 255.0, 1.0];
        const GROUND: [f32; 4] = [127.0 / 255.0, 55.0 / 255.0, 14.0 / 255.0, 1.0];

        let ground: types::Rectangle = [0.0, 256.0, 512.0, 256.0];

        gl.draw(args.viewport(), |c, gl| {
            clear(BLUE, gl);
            rectangle(GROUND, ground, c.transform, gl);
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tank {
    position: Point,
    cannon_angle: f32,
    health: f32,
}

impl Tank {
    fn new(position: Point) -> Tank {
        Tank {
            position,
            cannon_angle: 0f32,
            health: 100f32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point(f32, f32);
