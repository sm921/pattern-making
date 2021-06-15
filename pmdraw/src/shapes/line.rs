use super::{bezier::Bezier, point::Point};
use pmmath::matrix::Mat;
use std::{f64::consts::PI, ops};
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
    /// get angle in degree
    pub fn angle(&self) -> f64 {
        let vec = self.vec();
        let slope = vec.y / vec.x;
        slope.atan() / PI * 180.0
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

    /// Get point such that `(1-t) origin + t end`
    pub fn between(&self, t: f64) -> Point {
        self.origin.between(self.end, t)
    }

    pub fn extend_end(&mut self, length: f64) -> () {
        self.end = self.end + (length / self.len()) * self.vec()
    }

    pub fn extend_origin(&mut self, length: f64) -> () {
        self.origin = self.origin - (length / self.len()) * self.vec()
    }

    /// Get point where two lines intersect
    ///
    /// f(t) = t1 * a1 - t2 * a2 = c
    ///     where
    ///         a1 = end1 - origin1
    ///         a2 = end2 - origin2
    ///         b = -origin1 + origin2
    pub fn intersection(&self, l: &Line) -> Point {
        // vertical
        if self.vec().x == 0.0 {
            return l.at_x(self.origin.x);
        }
        // horizontal
        if self.vec().y == 0.0 {
            return l.at_y(self.origin.y);
        }

        let mut a = Mat::new(2, 2);
        let a1 = self.end - self.origin;
        let a2 = l.end - l.origin;
        a[0][0] = a1.x;
        a[0][1] = -a2.x;
        a[1][0] = a1.y;
        a[1][1] = -a2.y;
        let mut b = Mat::new(2, 1);
        let c = l.origin - self.origin;
        b[0][0] = c.x;
        b[1][0] = c.y;
        let solve = a.inverse().unwrap() * b;
        // use smaller t to reduce error
        let t1 = solve[0][0];
        let t2 = solve[1][0];
        if t1 > t2 {
            l.between(t2)
        } else {
            self.between(t1)
        }
    }

    /// Join two lines by extending both of them
    pub fn join(&mut self, l: &mut Line) -> () {
        let intersection = self.intersection(l);
        let extend = |l: &mut Line| {
            let d_origin = l.origin.distance(intersection);
            let d_end = l.end.distance(intersection);
            if d_origin < d_end {
                l.origin = intersection;
            } else {
                l.end = intersection;
            }
        };
        extend(self);
        extend(l);
    }

    pub fn join_bezier(&mut self, b: &mut Bezier) -> Line {
        let p = b.point_at(b.t_range().from + 0.01);
        let mut bezier_edge = b.range.from.line_to(p);
        self.join(&mut bezier_edge);
        bezier_edge
    }

    pub fn midddle(&self) -> Point {
        self.origin.middle(self.end)
    }

    pub fn mirror(&self, mirror_line: Line) -> Line {
        let mut mirrored_l = self
            .origin
            .mirror(mirror_line)
            .line_to(self.end.mirror(mirror_line));
        mirrored_l.reverse();
        mirrored_l
    }

    pub fn new(origin: Point, end: Point) -> Line {
        Line { origin, end }
    }

    /// Length of line
    pub fn len(&self) -> f64 {
        (&self.end - &self.origin).norm()
    }

    pub fn parallel(&self, distance: f64) -> Parallel {
        let parall_line = |is_left: bool| -> Line {
            let mut parallel_origin = self.point_from_origin(distance);
            parallel_origin.rotate(if is_left { 90.0 } else { -90.0 }, self.origin);
            let mut parallel_end = self.point_from_end(distance);
            parallel_end.rotate(if is_left { -90.0 } else { 90.0 }, self.end);
            parallel_origin.line_to(parallel_end)
        };
        Parallel {
            left: parall_line(true),
            right: parall_line(false),
        }
    }

    /// Get a point on this line by specifying distance from the line's end
    pub fn point_from_end(&self, length: f64) -> Point {
        self.end.to_point(self.origin, length)
    }

    /// Get a point on this line by specifying distance from the line's origin
    pub fn point_from_origin(&self, length: f64) -> Point {
        self.origin.to_point(self.end, length)
    }

    pub fn reverse(&mut self) -> () {
        let origin = self.origin.clone();
        self.origin = self.end.clone();
        self.end = origin;
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

    pub fn vec(&self) -> Point {
        self.end - self.origin
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

impl_op_ex_commutative!(*|scalar: f64, l: &Line| -> Line {
    l.origin.line_to(l.point_from_origin(l.len() * scalar))
});
