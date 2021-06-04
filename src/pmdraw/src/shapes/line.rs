use super::point::Point;

#[derive(Copy, Clone)]
pub struct Line {
    pub origin: Point,
    pub end: Point,
}

impl Line {
    pub fn new(origin: Point, end: Point) -> Line {
        Line { origin, end }
    }

    /// Length of line
    pub fn len(&self) -> f64 {
        (&self.end - &self.origin).norm()
    }

    /// Get a point on this line by specifying distance from the line's end
    pub fn point_from_end(&self, length: f64) -> Point {
        self.end.to_point(self.origin, length)
    }

    /// Get a point on this line by specifying distance from the line's origin
    pub fn point_from_origin(&self, length: f64) -> Point {
        self.origin.to_point(self.end, length)
    }
}
