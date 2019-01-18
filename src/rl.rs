extern crate nalgebra;

use nalgebra::Vector2;
use crate::world::*;
use crate::geometry_helper::*;

pub trait Environment {
    fn step(&mut self) -> f32;
}

impl Environment for World {
    fn step(&mut self) -> f32 {
        match self.tick() {
            Ok(_) => {
                let robot = self.robot.borrow();
                let position = &robot.position;
                if position.distance_to(&Vector2::new(1.0, 1.0)) < 0.2 && robot.speed.length() == 0.0 {
                    1.0
                } else { 0.0 }
            },
            Err(collision) => match collision {
                Collision::Crash(round) => {
                    -1.0
                },
                Collision::Lost => {
                    -1.0
                }
            }
        }
    }
}
