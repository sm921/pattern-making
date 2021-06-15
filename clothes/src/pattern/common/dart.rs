use pmdraw::shapes::{line::Line, point::Point};

use crate::pattern::measurements::Cm;

#[derive(Clone)]
pub struct Dart {
    pub fst: Line,
    pub snd: Line,
}

impl Dart {
    pub fn middle(&self) -> Point {
        self.fst.end
    }

    /// Create dart from 3 points
    /// - fst - first line's edge point
    /// - snd - second line's edge point
    /// - middle - common edge point of the fist and second lines
    pub fn new(fst_edge: Point, middle: Point, snd_edge: Point) -> Dart {
        Dart {
            fst: Line::new(fst_edge, middle),
            snd: Line::new(middle, snd_edge),
        }
    }

    /// move by (dx, dy)
    pub fn to(&mut self, dx: Cm, dy: Cm) {
        self.fst.to(dx, dy);
        self.snd.to(dx, dy);
    }

    pub fn width(&self) -> Cm {
        self.fst.origin.distance(self.snd.end)
    }
}
