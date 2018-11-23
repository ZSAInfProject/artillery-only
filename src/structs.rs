use bit_vec::BitVec;
use std::collections::HashMap;

pub struct Map {
    width: usize,
    height: usize,
    terrain: BitVec,
    tanks: HashMap<u32, Tank>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut terrain = BitVec::from_elem(width * height, false);
        for i in 0..(width * height / 2) {
            terrain.set(i, true);
        }
        Map {
            width,
            height,
            terrain,
            tanks: HashMap::new(),
        }
    }

    fn get_tile(self, x: usize, y: usize) -> Option<bool> {
        return self.terrain.get(y * self.width + x);
    }
    fn set_tile(mut self, is_solid: bool, x: usize, y: usize) {
        self.terrain.set(y * self.width + x, is_solid);
    }
}

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

struct Point(f32, f32);
