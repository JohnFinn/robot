extern crate rand;
extern crate ggez;

use std::cell::RefCell;
use rand::Rng;
use ggez::graphics::Vector2;

use crate::world::*;

pub struct DrunkPilot{
    generator: RefCell<rand::prelude::ThreadRng>
}

impl DrunkPilot {
    pub fn new() -> DrunkPilot {
        DrunkPilot{generator: RefCell::new(rand::thread_rng())}
    }
}

impl Pilot for DrunkPilot {
    fn throttle(&self, world: &World) -> Vector2 {
        let mut generator = self.generator.borrow_mut();
        let x = generator.gen_range(-0.01, 0.01);
        let y = generator.gen_range(-0.01, 0.01);
        Vector2::new(x,y)
    }
}
