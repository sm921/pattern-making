use pmdraw::shapes::{bezier::Bezier, line::Line};

use crate::pattern::{common::dart::Dart, measurements::Cm};

pub struct Front {
    pub center: Line,
    pub arm_hole: (Bezier, Bezier),
    pub dart: Dart,
    pub neck: Bezier,
    pub chest_dart: Dart,
    pub shoulder: Line,
    pub side: Line,
    pub side_dart: Dart,
    pub waist: Line,
}

impl Front {
    fn for_each_bezier_mut<T>(&mut self, mut callback: T)
    where
        T: FnMut(&mut Bezier) -> (),
    {
        for bezier in vec![&mut self.arm_hole.0, &mut self.arm_hole.1, &mut self.neck] {
            callback(bezier)
        }
    }

    fn for_each_line_mut<T>(&mut self, mut callback: T)
    where
        T: FnMut(&mut Line) -> (),
    {
        for line in vec![
            &mut self.center,
            &mut self.chest_dart.fst,
            &mut self.chest_dart.snd,
            &mut self.dart.fst,
            &mut self.dart.snd,
            &mut self.shoulder,
            &mut self.side,
            &mut self.side_dart.fst,
            &mut self.side_dart.snd,
            &mut self.waist,
        ] {
            callback(line)
        }
    }

    pub fn to(&mut self, dx: Cm, dy: Cm) {
        self.for_each_bezier_mut(|b| b.to(dx, dy));
        self.for_each_line_mut(|l| l.to(dx, dy));
    }
}
