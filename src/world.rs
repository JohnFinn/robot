extern crate nalgebra;

use std::cell::RefCell;

type Vector2 = nalgebra::Vector2<f32>;

pub struct Robot {
    pub position: nalgebra::Vector2<f32>,
    pub speed: nalgebra::Vector2<f32>
}

pub trait Pilot{
    fn throttle(&self, world: &World) -> nalgebra::Vector2<f32>;
}

impl Robot {
    pub fn push(&mut self, force: &nalgebra::Vector2<f32>, time: f32){
        self.position += self.speed * time;
        self.speed += force * time;
    }
}

pub struct Round{
    pub center: Vector2,
    pub radius: f32,
}

impl Round {
    pub fn new(x: f32, y: f32, r:f32) -> Round {
        Round{center:Vector2::new(x,y), radius: r}
    }
}

pub struct World<'a> {
    pub rounds: Vec<Round>,
    pub robot: RefCell<Robot>,
    pub pilot : &'a dyn Pilot,
    pub max_force: f32,
    pub time: f32,
}

pub enum Collision<'a>{
    Crash(&'a Round),
    Lost
}

impl<'a> World<'a> where {
    pub fn new(rounds: Vec<Round>, pilot: &'a dyn Pilot, max_force: f32, time: f32) -> World<'a> {
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

    pub fn tick(&mut self) -> Result<(), Collision> {
        if !self.check_borders() {
            return Err(Collision::Lost);
        }
        if let Some(round) = self.check_collisions() {
            return Err(Collision::Crash(round));
        }
        let force = self.pilot.throttle(self);
        self.robot.borrow_mut().push(&force, self.time);
        return Ok(());
    }
}
