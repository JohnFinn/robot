extern crate nalgebra;

use num_traits::pow::Pow;

type Vector2 = nalgebra::Vector2<f32>;

pub trait Rotatable{
    fn rotate180(&mut self);
    fn rotate90left(&mut self);
    fn rotate_left(&mut self, angle: f32);
}

pub trait Length{
    fn length(&self) -> f32;
}

pub trait Distance{
    fn distance_to(&self, other: &Self) -> f32;
}

impl Rotatable for nalgebra::Vector2<f32> {
    fn rotate180(&mut self){
        self.x = -self.x;
        self.y = -self.y;
    }

    fn rotate90left(&mut self){
        let (x, y) = (-self.y, self.x);
        self.x = x;
        self.y = y;
    }

    fn rotate_left(&mut self, angle: f32){
        let sin = angle.sin();
        let cos = angle.cos();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        self.x = x;
        self.y = y;
    }
}

impl Distance for nalgebra::Vector2<f32>{
    fn distance_to(&self, other: &Vector2) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt()
    }
}

impl Length for nalgebra::Vector2<f32>{
    fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}


#[cfg(test)]
mod test{
    use nalgebra::Vector2;
    use super::*;

    #[test]
    fn test_rotate90left(){
        let mut vec = Vector2::new(1.0, 2.0);
        vec.rotate90left();
        assert_eq!(vec, Vector2::new(-2.0, 1.0));
    }

    #[test]
    fn test_rotate180(){
        let mut vec = Vector2::new(1.0, 2.0);
        vec.rotate180();
        assert_eq!(vec, Vector2::new(-1.0, -2.0));
    }

    #[test]
    fn test_rotate(){
        let mut vec = Vector2::new(3_f32.sqrt()/2.0, 0.5);
        vec.rotate_left(std::f64::consts::PI as f32 / 6.0);
        assert_eq!(vec, Vector2::new(0.5, 3_f32.sqrt()/2.0));
    }

    #[test]
    fn test_length(){
        let v = Vector2::new(1.0, 1.0);
        assert_eq!(v.length(), 2_f32.sqrt());
    }

    #[test]
    fn test_distance_to(){
        let a = Vector2::new(1.0, 1.0);
        let b = Vector2::new(3.0, 1.0);
        assert_eq!(a.distance_to(&b), 2.0);
    }
}
