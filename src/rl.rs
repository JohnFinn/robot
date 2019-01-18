extern crate nalgebra;

use nalgebra::Vector2;
use crate::world::*;

pub trait Environment {
    fn step(&mut self) -> f32;
    fn reset(&mut self);
    fn done(&self) -> bool;
}

impl Environment for World {
    fn step(&mut self) -> f32 {
        self.tick();
        match self.at() {
            Ok(position) => match position {
                Position::Finish => 1.0,
                Position::Flight => 0.0
            },
            Err(_) => -1.0
        }
    }

    fn reset(&mut self){
        self.robot.borrow_mut().speed = Vector2::new(0.0, 0.0);
        self.robot.borrow_mut().position = Vector2::new(0.0, 0.0);
    }

    fn done(&self) -> bool {
        match self.at() {
            Ok(position) => match position {
                Position::Flight => false,
                Position::Finish => true
            }
            Err(_) => true
        }
    }
}
