use std::f64::consts::PI;

use super::point::Point;

#[derive(Copy, Clone)]
pub struct Circle {
    pub origin: Point,
    pub r: f64,
}

impl Circle {
    pub fn new(origin: Point, r: f64) -> Circle {
        Circle { origin, r }
    }

    pub fn point_at(&self, angle_degree: f64) -> Point {
        let theta = PI / 180.0 * angle_degree;
        self.origin + (self.r * Point::new(theta.cos(), theta.sin()))
    }
}
