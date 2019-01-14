extern crate ggez;
extern crate num_traits;

use ggez::graphics::Vector2;
use ggez::nalgebra;

use num_traits::pow::Pow;

use crate::world::*;

pub struct Pilot1{

}

impl Pilot1 {
    pub fn new() -> Pilot1 {
        Pilot1{}
    }
}

fn rotate90left(vec: &Vector2) -> Vector2 {
    Vector2::new(-vec.y, vec.x)
}

fn rotate180(v: &Vector2) -> Vector2 {
    Vector2::new(-v.x, -v.y)
}

fn distance(a: &Vector2, b: &Vector2) -> f32 {
    (((a.x - b.x).pow(2) + (b.x - b.y).pow(2)) as f32).sqrt()
}

fn length(a: &Vector2) -> f32 {
    ((a.x * a.x) + (a.y * a.y)).sqrt()
}

fn reward(speed_to_center: f32, distance: f32) -> f32 {
    speed_to_center / distance * 0.0001
}

// TODO may be possible to optimize 'cause we need only sign of dot product
fn force_around(robot: &Robot, round: &Round) -> Vector2 {
    let to_center = round.center - robot.position;
    // if robot doesn't move towards the round
    if robot.speed.dot(&to_center) <= 0.0 {
        return Vector2::new(0.0, 0.0);
    }
    let mut to_way_around = rotate90left(&to_center);
    if robot.speed.dot(&to_way_around) < 0.0 {
        to_way_around = rotate180(&to_way_around);
    }
    to_way_around
}

impl Pilot for Pilot1 {
    fn throttle(&self, world: &World) -> Vector2 {
        let robot = world.robot.borrow();
        let mut rewards = (Vector2::new(1.0, 1.0) - robot.position) * 0.0001;
        for round in world.rounds.iter() {
            let to_way_around = force_around(&robot, &round);
            if length(&to_way_around) == 0.0 {
                continue;
            }
            let to_center = round.center - robot.position;
            let speed_to_center = nalgebra::dot(&robot.speed, &to_center);
            let k = reward(speed_to_center, distance(&round.center, &robot.position) - round.radius);
            rewards += nalgebra::normalize(&to_way_around) * k;
        }
        rewards
    }
}
