extern crate rand;
extern crate nalgebra;

type Vector2 = nalgebra::Vector2<f32>;

use std::cell::RefCell;
use rand::Rng;

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
    fn throttle(&self, world: &World) -> nalgebra::Vector2<f32> {
        let mut generator = self.generator.borrow_mut();
        let x = generator.gen_range(-0.001, 0.001);
        let y = generator.gen_range(-0.001, 0.001);
        Vector2::new(x,y)
    }
}
