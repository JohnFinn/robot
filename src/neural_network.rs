extern crate nalgebra;

pub use nalgebra::*;
use num_traits::pow::Pow;

#[derive(Debug)]
pub struct Net1{
    pub layer1: DMatrix<f32>,
    pub layer2: DMatrix<f32>
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

fn cost(result: (f32, f32), expected: (f32, f32)) -> f32 {
    (result.0 - expected.0).pow(2) + (result.1 - expected.1).pow(2)
}

impl Net1 {
    pub fn new() -> Net1 {
        Net1{
            layer1: DMatrix::new_random(4, 7),
            layer2: DMatrix::new_random(2, 4)
        }
    }

    pub fn get(&self, input: &DVector<f32>) -> (f32, f32) {
        let hidden = &self.layer1 * input;
        let result = &self.layer2 * &hidden;
        (sigmoid(result[0]), sigmoid(result[1]))
    }

    pub fn backprop(expected: (f32, f32), actual: (f32, f32)) -> (DMatrix<f32>, DMatrix<f32>) {
        let d1 = DMatrix::from_element(7, 4, 0.0);
        let d2 = DMatrix::from_element(2, 4, 0.0);
        (d1, d2)
    }
}
