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

    /// Get point such that `(1-t) origin + t end`
    pub fn between(&self, t: f64) -> Point {
        self.origin.between(self.end, t)
    }

    pub fn midddle(&self) -> Point {
        self.origin.middle(self.end)
    }

    pub fn new(origin: Point, end: Point) -> Line {
        Line { origin, end }
    }

    /// Length of line
    pub fn len(&self) -> f64 {
        (&self.end - &self.origin).norm()
    }

    pub fn parallel(&self, distance: f64) -> Parallel {
        let line_as_vector = self.end - self.origin;
        let d = if line_as_vector.y == 0.0 {
            Point::new(0.0, -distance)
        } else {
            let vertical_slope = -line_as_vector.x / line_as_vector.y;
            // dx = distance * cos(theta)
            //  where theta = 1 / root(1 + slope^2)
            let dx = distance * (1.0 + vertical_slope.powf(2.0)).sqrt();
            // dy = distance * sin(theta)
            //  where theta = slope / root(1 + slope^2)
            let dy = distance * vertical_slope * (1.0 + vertical_slope.powf(2.0)).sqrt();
            Point::new(dx, dy)
        };
        let mut left = self.clone();
        left.to(-d.x, -d.y);
        let mut right = self.clone();
        right.to(d.x, d.y);
        Parallel { left, right }
    }

    /// Get a point on this line by specifying distance from the line's end
    pub fn point_from_end(&self, length: f64) -> Point {
        self.end.to_point(self.origin, length)
    }

    /// Get a point on this line by specifying distance from the line's origin
    pub fn point_from_origin(&self, length: f64) -> Point {
        self.origin.to_point(self.end, length)
    }

    /// Rotate around point
    pub fn rotate(&mut self, angle_degree: f64, point: Point) -> () {
        self.origin.rotate(angle_degree, point);
        self.end.rotate(angle_degree, point);
    }

    /// Split the line at a point and get two separate lines
    pub fn split_at_x(&self, x: f64) -> Split {
        let split_point = self.at_x(x);
        Split {
            fst: Line::new(self.origin, split_point),
            snd: Line::new(split_point, self.end),
        }
    }

    /// move line
    pub fn to(&mut self, dx: f64, dy: f64) {
        self.origin = self.origin.to(dx, dy);
        self.end = self.end.to(dx, dy);
    }
}

/// use this struct instead of tuple 'cause wasm is not capable of tuple
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Split {
    pub fst: Line,
    pub snd: Line,
}

pub struct Parallel {
    pub left: Line,
    pub right: Line,
}
