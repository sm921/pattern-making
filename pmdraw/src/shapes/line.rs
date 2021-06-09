use super::point::Point;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone)]
pub struct Line {
    pub origin: Point,
    pub end: Point,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Line {
    pub fn new(origin: Point, end: Point) -> Line {
        Line { origin, end }
    }

    /// Length of line
    pub fn len(&self) -> f64 {
        (&self.end - &self.origin).norm()
    }

    /// Get the point on the line by x
    pub fn at_x(&self, x: f64) -> Point {
        let t = (x - self.origin.x) / (self.end.x - self.origin.x);
        (1.0 - t) * self.origin + t * self.end
    }

    /// Get the point on the line by y
    pub fn at_y(&self, y: f64) -> Point {
        let t = (y - self.origin.y) / (self.end.y - self.origin.y);
        (1.0 - t) * self.origin + t * self.end
    }

    /// Get a point on this line by specifying distance from the line's end
    pub fn point_from_end(&self, length: f64) -> Point {
        self.end.to_point(self.origin, length)
    }

    /// Get a point on this line by specifying distance from the line's origin
    pub fn point_from_origin(&self, length: f64) -> Point {
        self.origin.to_point(self.end, length)
    }

    /// move line
    pub fn to(&mut self, dx: f64, dy: f64) {
        self.origin = self.origin.to(dx, dy);
        self.end = self.origin.to(dx, dy);
    }
}
