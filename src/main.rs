extern crate ggez;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate nalgebra;
extern crate nn;

use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

mod world_draw;
mod world;
mod pilots;
mod geometry_helper;
mod rl;

use self::rl::*;
use self::world::*;
use self::pilots::*;
use self::world_draw::*;

use nn::{NN, HaltCondition};

fn discount_rewards(vec: &Vec<f32>) -> Vec<f32> {
    if vec.len() == 0 {
        return Vec::new();
    }
    let mut result = Vec::new();
    result.resize(vec.len(), 0.0);
    let gamma = 0.99;
    let mut current = 0.0;
    for index in vec.len()-1..=0 {
        current = current * gamma + vec[index];
        result[index] = current;
    }
    result
}

fn train(world: &mut World, pilot: &Pilot1, net: Rc<RefCell<nn::NN>>, renderer: &WorldRenderer){
    let mut reward = 0.0;
    for i in 0..100 {
        reward = generate_training_data(world, pilot, renderer);
        world.reset();
    }
    {
        let mut training_data = pilot.training_data.borrow_mut();
        net.borrow_mut()
            .train(&training_data[..])
            .halt_condition( HaltCondition::Epochs(500) )
            .log_interval( Some(100) )
            .momentum( 0.1 )
            .rate( 0.3 )
            .go();
        training_data.clear();
    }
}

fn generate_training_data(world: &mut World, pilot: &Pilot1, renderer: &WorldRenderer) -> f32 {
    let mut rewards = vec![];
    let mut total_reward = 0.0;
    loop {
        let reward = world.step();
        total_reward += reward;
        if world.done() {
            print!("");
            break;
        }
        rewards.push(reward);
        renderer.move_robot(&world).unwrap();
    }
    let rewards = discount_rewards(&rewards);
    for ((_input, result), reward) in pilot.training_data.borrow_mut().iter_mut().zip(rewards) {
        // reward - 1 оставляем как есть, -1 изменяем как можно сильнее
        let badness = (1.0 - reward as f64)/2.0;
        result[0] = wiggle(result[0], badness);
        result[1] = wiggle(result[1], badness);
    }
    total_reward
}

fn wiggle(value: f64, badness: f64) -> f64 {
    let mut gen = rand::thread_rng();
    let left = value - value * badness;
    let right = value + (1.0 -value) * badness;
    gen.gen_range(left, right)
}


fn main() {

    let net = NN::new(&[7, 4, 2]);
    let net = Rc::new(RefCell::new(net));
    let pilot = Rc::new(Pilot1::new(net.clone()));

    let rounds = vec![
        Round::new(0.8, 0.7, 0.01)
    ];
    let mut world = World::new(rounds, pilot.clone(), 0.001, 1.0);

    let renderer = WorldRenderer::new();
    renderer.init_with(&world).unwrap();
    loop {
        train(&mut world, &pilot, net.clone(), &renderer);
    }
}
