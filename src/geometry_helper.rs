extern crate nalgebra;

use num_traits::pow::Pow;

type Vector2 = nalgebra::Vector2<f32>;

pub fn rotate90left(vec: &Vector2) -> Vector2 {
    Vector2::new(-vec.y, vec.x)
}

pub fn rotate180(v: &Vector2) -> Vector2 {
    Vector2::new(-v.x, -v.y)
}

pub fn distance(a: &Vector2, b: &Vector2) -> f32 {
    (((a.x - b.x).pow(2) + (b.x - b.y).pow(2)) as f32).sqrt()
}

pub fn length(a: &Vector2) -> f32 {
    ((a.x * a.x) + (a.y * a.y)).sqrt()
}

pub fn rotate(a: &Vector2, angle: f32) -> Vector2 {
    let sin = angle.sin();
    let cos = angle.cos();
    let x = a.x * cos - a.y * sin;
    let y = a.x * sin - a.y * cos;
    Vector2::new(x, y)
}
