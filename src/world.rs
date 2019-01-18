extern crate nalgebra;

use std::cell::RefCell;
use std::rc::Rc;

use crate::geometry_helper::*;

type Vector2 = nalgebra::Vector2<f32>;

pub struct Robot {
    pub position: Vector2,
    pub speed: Vector2
}

pub trait Pilot{
    fn throttle(&self, world: &World) -> Vector2;
}

impl Robot {
    pub fn push(&mut self, force: &Vector2, time: f32){
        self.position += self.speed * time;
        self.speed += force * time;
    }
}

#[derive(Clone, Copy)]
pub struct Round{
    pub center: Vector2,
    pub radius: f32,
}

impl Round {
    pub fn new(x: f32, y: f32, r:f32) -> Round {
        Round{center:Vector2::new(x,y), radius: r}
    }
}

pub struct World {
    pub rounds: Vec<Round>,
    pub robot: RefCell<Robot>,
    pub pilot : Rc<Pilot>,
    pub max_force: f32,
    pub time: f32,
}

pub enum Collision<'a>{
    Crash(&'a Round),
    Lost
}

pub enum Position{
    Finish, Flight
}

impl World {
    pub fn new(rounds: Vec<Round>, pilot: Rc<dyn Pilot>, max_force: f32, time: f32) -> World {
        World{
            rounds: rounds,
            robot: RefCell::new(Robot{
                position: Vector2::new(-0.9,-0.9),
                speed: Vector2::new(0.0, 0.0)
            }),
            pilot: pilot,
            max_force: max_force,
            time: time
        }
    }

    fn check_collisions(&self) -> Option<&Round> {
        let position = self.robot.borrow().position;
        for round in self.rounds.iter() {
            let r = round.center - position;
            if r.x * r.x + r.y * r.y <= round.radius * round.radius {
                return Some(&round);
            }
        }
        return None;
    }

    pub fn check_borders(&self) -> bool {
        let position = self.robot.borrow().position;
        let (x, y) = (position.x, position.y);
        x >= -1.0 && x <= 1.0 && y >= -1.0 && y <= 1.0
    }

    pub fn at(&self) -> Result<Position, Collision> {
        if !self.check_borders() {
            return Err(Collision::Lost);
        }
        if let Some(round) = self.check_collisions() {
            return Err(Collision::Crash(round));
        }
        let robot = self.robot.borrow();
        if robot.position.distance_to(&Vector2::new(1.0, 1.0)) < 0.2 && robot.speed.length() < 0.001 {
            return Ok(Position::Finish);
        }
        Ok(Position::Flight)
    }

    pub fn tick(&mut self) -> Result<Position, Collision> {
        let position = self.at()?;
        let force = self.pilot.throttle(self);
        self.robot.borrow_mut().push(&force, self.time);
        return Ok(position);
    }
}
