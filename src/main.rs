extern crate ggez;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate nalgebra;
extern crate nn;

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

type Vector2 = nalgebra::Vector2<f32>;

use ggez::{ContextBuilder, conf, event};

mod world_draw;
mod world;
mod pilots;
mod neural_network;
mod geometry_helper;
mod rl;

use self::rl::*;
use self::world::*;
use self::pilots::*;
use self::geometry_helper::*;
use self::world_draw::*;

use nn::{NN, HaltCondition};

fn generate_training_data(world: &mut World, pilot: &Pilot1, renderer: &WorldRenderer){
    let mut gen = rand::thread_rng();
    for i in 0..100 {
        let lastlen = pilot.training_data.borrow().len();
        let mut counter = 0;
        loop {
            let reward = world.step();
            renderer.move_robot(&world);
            match world.at() {
                Ok(result) => match result {
                    Position::Flight => {
                        counter += 1;
                        if counter > 10 {
                            break;
                        }
                    },
                    Position::Finish => {
                        break;
                    },
                },
                Err(_) => {
                    // for i in pilot
                    break;
                }
            }
            let mut robot = world.robot.borrow_mut();
            robot.position *= 0.0;
            robot.speed *= 0.0;

            for (input, result) in pilot.training_data.borrow_mut().iter_mut().skip(lastlen) {
                result[0] = gen.gen_range(0.0, 1.0);
                result[1] = 0.0;
            }
        }
    }
}

fn main() {
    let net = Rc::new(RefCell::new(NN::new(&[7, 4, 2])));
    let rounds = vec![
        Round::new(0.8, 0.7, 0.01)
    ];
    let mut pilot = Rc::new(Pilot1::new(net.clone()));
    let p2 = Rc::new(DrunkPilot::new());
    let mut world = World::new(rounds, p2.clone(), 0.001, 1.0);
    let renderer = WorldRenderer::new();
    renderer.init_with(&world);
    generate_training_data(&mut world, &pilot, &renderer);
    {
        let examples = pilot.training_data.borrow();
        net.borrow_mut().train(&examples[..])
            .halt_condition( HaltCondition::Epochs(10000) )
            .log_interval( Some(1000) )
            .momentum( 0.1 )
            .rate( 0.3 )
            .go();
    }
}
