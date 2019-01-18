extern crate num_traits;
extern crate nalgebra;
extern crate nn;

use std::cell::RefCell;
use std::rc::Rc;

use crate::world::*;
use crate::geometry_helper::*;

type Vector2 = nalgebra::Vector2<f32>;

pub struct Pilot1{
    pub net: Rc<RefCell<nn::NN>>,
    pub training_data: Rc<RefCell<Vec<(Vec<f64>, Vec<f64>)>>>
}

impl Pilot1 {
    pub fn new(net: Rc<RefCell<nn::NN>>) -> Pilot1 {
        Pilot1{
            net: net, training_data: Rc::new(RefCell::new(Vec::new()))
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

impl Pilot for Pilot1 {
    fn throttle(&self, world: &World) -> Vector2 {
        let robot = world.robot.borrow();

        let r = &world.rounds[0];
        let input = vec![
            robot.position.x as f64, robot.position.y as f64,
            robot.speed.x as f64, robot.speed.y as f64,
            r.center.x as f64, r.center.y as f64, r.radius as f64
        ];
        let result = self.net.borrow_mut().run(&input);
        let (direction, force) = (result[0] as f32, result[1] as f32);
        self.training_data.borrow_mut().push((input, result));

        let angle = 2.0 * std::f64::consts::PI as f32 * direction;
        let component = ((0.001 * force).sqrt()/2.0).sqrt();
        let mut force = Vector2::new(component, component);
        force.rotate_left(angle);
        force
    }
}
