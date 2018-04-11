extern crate nalgebra;
extern crate ncollide_geometry;

use nalgebra::Vector2;
use ncollide_geometry::shape::{Ball2, Cuboid2, Shape2};

fn main() {
    println!("Hello, world!");
}

trait Collidable {
    fn to_shape(&self) -> Box<Shape2<f32>>;
}

struct Rect {}

impl Collidable for Rect {
    fn to_shape(&self) -> Box<Cuboid2<f32>> {
        Box::new(Cuboid2::new(Vector2::new(2.0, 2.0)))
    }
}

struct Ball {}

impl Collidable for Ball {
    fn to_shape(&self) -> Box<Ball2<f32>> {
        Box::new(Ball2::new(2.0))
    }
}
