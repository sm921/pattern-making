use pmdraw::shapes::{line::Line, point::Point};

use crate::pattern::measurements::Cm;

pub struct Dart {
    pub fst: Line,
    pub snd: Line,
}

impl Dart {
    /// Create dart from 3 points
    /// - fst - first line's edge point
    /// - snd - second line's edge point
    /// - middle - common edge point of the fist and second lines
    pub fn new(fst: Point, middle: Point, snd: Point) -> Dart {
        Dart {
            fst: Line::new(middle, fst),
            snd: Line::new(middle, snd),
        }
    }

    /// move by (dx, dy)
    pub fn to(&mut self, dx: Cm, dy: Cm) {
        self.fst.to(dx, dy);
        self.snd.to(dx, dy);
    }
}
