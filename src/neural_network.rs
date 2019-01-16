extern crate nalgebra;

pub use nalgebra::*;

#[derive(Debug)]
pub struct Net1{
    pub layer1: DMatrix<f32>,
    pub layer2: DMatrix<f32>
}

fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

impl Net1{
    pub fn new() -> Net1 {
        let gen = rand::thread_rng();
        Net1{
            layer1: DMatrix::new_random(4, 7),
            layer2: DMatrix::new_random(2, 4)
        }
    }

    pub fn get(&self, input: &DVector<f32>) -> (f32, f32) {
        let hidden = &self.layer1 * input;
        let result = &self.layer2 * &hidden;
        // let (a,b) = result.shape();
        // println!("{} {}", a, b);
        (sigmoid(result[0]), sigmoid(result[1]))
    }
}
