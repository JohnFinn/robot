extern crate num_traits;
extern crate nalgebra;

use crate::world::*;
use crate::geometry_helper::*;
use crate::neural_network::Net1;

type Vector2 = nalgebra::Vector2<f32>;

pub struct Pilot1<'a>{
    pub net: &'a Net1
}

impl<'a> Pilot1<'a> {
    pub fn new(net: &'a Net1) -> Pilot1<'a> {
        Pilot1{
            net: net
        }
    }
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
    let mut to_way_around = to_center.clone();
    to_way_around.rotate90left();

    if robot.speed.dot(&to_way_around) < 0.0 {
        to_way_around.rotate180();
    }
    to_way_around
}

impl<'a> Pilot for Pilot1<'a> {
    fn throttle(&self, world: &World) -> Vector2 {
        let robot = world.robot.borrow();
        /*
        let mut rewards = (Vector2::new(1.0, 1.0) - robot.position) * 0.0001;
        for round in world.rounds.iter() {
            let to_way_around = force_around(&robot, &round);
            if length(&to_way_around) == 0.0 {
                continue;
            }
            let to_center = round.center - robot.position;
            let speed_to_center = nalgebra::dot(&robot.speed, &to_center);
            let k = reward(speed_to_center, length(&to_center) - round.radius);
            rewards += nalgebra::normalize(&to_way_around) * k;
        }
        rewards
        */
        let r = &world.rounds[0];
        let (direction, force) = self.net.get(&nalgebra::DVector::from_row_slice(7, &[
            robot.position.x, robot.position.y,
            robot.speed.x, robot.speed.y,

            r.center.x, r.center.y, r.radius
        ]));

        let angle = 2.0 * std::f64::consts::PI as f32 * direction;
        let component = (force.sqrt()/2.0).sqrt();
        let mut force = Vector2::new(component, component);
        force.rotate_left(angle);
        force
    }
}
