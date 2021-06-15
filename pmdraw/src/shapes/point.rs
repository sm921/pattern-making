use std::ops;

use std::f64::consts::PI;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use super::line::Line;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Point {
    pub fn between(&self, another: Point, t: f64) -> Point {
        (1.0 - t) * self.clone() + t * another
    }

    pub fn distance(&self, another: Point) -> f64 {
        self.line_to(another).len()
    }

    pub fn line_to(&self, point: Point) -> Line {
        Line::new(*self, point)
    }

    pub fn middle(&self, another: Point) -> Point {
        self.between(another, 0.5)
    }

    pub fn mirror(&self, mirror_line: Line) -> Point {
        let l_vec = mirror_line.vec();
        let vertical_direction = Point::new(-l_vec.y, l_vec.x);
        let vertical_line = self.line_to(self + vertical_direction);
        let vertical_point_on_the_mirror = mirror_line.intersection(&&vertical_line);
        if self == &vertical_point_on_the_mirror {
            vertical_point_on_the_mirror
        } else {
            (self.line_to(vertical_point_on_the_mirror) * 2.0).end
        }
    }

    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y, z: 0.0 }
    }

    pub fn new3(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    /// Distance from origin
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Get a new point whose norm is 1
    pub fn normalize(&self) -> Point {
        Point::new3(self.x, self.y, self.z) / self.norm()
    }

    /// Rotate around z axis
    pub fn rotate(&mut self, angle_degree: f64, origin: Point) -> () {
        self.rotate_around_any_axis(angle_degree, origin, Axis::Z)
    }

    pub fn rotate_around_any_axis(&mut self, angle_degree: f64, origin: Point, axis: Axis) -> () {
        let theta = PI / 180.0 * angle_degree;
        let sin = theta.sin();
        let cos = theta.cos();
        // relative to origin
        let mut p = self.clone() - origin;
        let (x, y, z) = (p.x, p.y, p.z);
        p = match axis {
            Axis::X => {
                p.y = cos * y - sin * z;
                p.z = sin * y + cos * z;
                p
            }
            Axis::Y => {
                p.z = cos * z - sin * x;
                p.x = sin * z + cos * x;
                p
            }
            Axis::Z => {
                p.x = cos * x - sin * y;
                p.y = sin * x + cos * y;
                p
            }
        };
        // original coordinates
        p = p + origin;
        self.x = p.x;
        self.y = p.y;
        self.z = p.z;
    }

    /// Create a new point relative to this point
    pub fn to(&self, dx: f64, dy: f64) -> Point {
        Point::new3(self.x + dx, self.y + dy, self.z)
    }
    pub fn to3(&self, dx: f64, dy: f64, dz: f64) -> Point {
        Point::new3(self.x + dx, self.y + dy, self.z + dz)
    }

    /// Create a new point relative to this point towards another point
    pub fn to_point(&self, towards: Point, length: f64) -> Point {
        let d = (towards - self).normalize() * length;
        self.to(d.x, d.y)
    }

    /// Create a new point relative to this point in angular coodinates
    pub fn to_angular(&self, angle_degree: f64, length: f64) -> Point {
        self.to_angular_around_any_axis(angle_degree, length, Axis::Z)
    }
    pub fn to_angular_around_any_axis(&self, angle_degree: f64, length: f64, axis: Axis) -> Point {
        let theta = PI / 180.0 * angle_degree;
        let d1 = length * theta.cos();
        let d2 = length * theta.sin();
        let (x, y, z) = (self.x, self.y, self.z);
        match axis {
            Axis::X => Point::new3(x, y + d1, z + d2),
            Axis::Y => Point::new3(x + d2, y, z + d1),
            Axis::Z => Point::new3(x + d1, y + d2, z),
        }
    }
}

impl_op_ex!(+|p1: &Point, p2: &Point| -> Point {
    Point::new3(p1.x +p2.x, p1.y+p2.y, p1.z+p2.z)
});
impl_op_ex!(-|p1: &Point, p2: &Point| -> Point {
    Point::new3(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z)
});
impl_op_ex_commutative!(*|scalar: f64, p: &Point| -> Point {
    Point::new3(p.x * scalar, p.y * scalar, p.z * scalar)
});
impl_op_ex!(/|p: &Point, scalar: f64| -> Point {
    Point::new3(p.x / scalar, p.y / scalar, p.z / scalar)
});

pub fn sigma<T>(a: T, from: usize, to: usize) -> Point
where
    T: Fn(usize) -> Point,
{
    let mut sum = Point::new(0.0, 0.0);
    for k in from..to {
        sum = sum + a(k);
    }
    sum
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
