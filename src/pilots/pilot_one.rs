extern crate num_traits;
extern crate nalgebra;
extern crate nn;
extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;

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

impl Pilot for Pilot1 {
    fn throttle(&self, world: &World) -> Vector2 {
        let robot = world.robot.borrow();

        let r = &world.rounds[0];
        let input = vec![
            robot.position.x as f64, robot.position.y as f64,
            robot.speed.x as f64, robot.speed.y as f64,
            r.center.x as f64, r.center.y as f64, r.radius as f64
        ];
        let mut result = self.net.borrow().run(&input);
        if result[0] > rand::thread_rng().gen_range(0.0, 1.0) {
            result[0] = rand::thread_rng().gen_range(0.0, 1.0);
        }
        if result[1] > rand::thread_rng().gen_range(0.0, 1.0) {
            result[1] = rand::thread_rng().gen_range(0.0, 1.0);
        }

        let (direction, force) = (result[0] as f32, result[1] as f32);
        self.training_data.borrow_mut().push((input, result));

        let angle = 2.0 * std::f64::consts::PI as f32 * direction;
        let component = ((0.000000001 * force).sqrt()/2.0).sqrt();
        let mut force = Vector2::new(component, component);
        force.rotate_left(angle);
        force
    }
}
