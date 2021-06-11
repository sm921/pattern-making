use pmdraw::shapes::{bezier::Bezier, line::Line, point::Point};

use crate::pattern::{common::dart::Dart, measurements::Cm};

pub struct Back {
    pub arm_hole: Bezier,
    pub dart1: Dart,
    pub dart2: Dart,
    pub neck: Bezier,
    pub side: Line,
    pub shoulder: Line,
    pub shoulder_dart: Dart,
    pub center: Line,
    pub center_dart: Line,
    pub waist: Line,
}

impl Back {
    fn for_each_bezier_mut<T>(&mut self, mut callback: T)
    where
        T: FnMut(&mut Bezier) -> (),
    {
        for bezier in vec![&mut self.arm_hole, &mut self.neck] {
            callback(bezier)
        }
    }

    fn for_each_line_mut<T>(&mut self, mut callback: T)
    where
        T: FnMut(&mut Line) -> (),
    {
        for line in vec![
            &mut self.dart1.fst,
            &mut self.dart1.snd,
            &mut self.dart2.fst,
            &mut self.dart2.snd,
            &mut self.center,
            &mut self.center_dart,
            &mut self.shoulder,
            &mut self.shoulder_dart.fst,
            &mut self.shoulder_dart.snd,
            &mut self.side,
            &mut self.waist,
        ] {
            callback(line)
        }
    }

    pub fn rotate(&mut self, angle_degree: f64, around: Point) {
        self.for_each_bezier_mut(|b| b.rotate(angle_degree, around));
        self.for_each_line_mut(|l| l.rotate(angle_degree, around));
    }

    pub fn rotate_around_center(&mut self, angle_degree: f64) {
        let center = self
            .waist
            .midddle()
            .to(0.0, self.waist.end.middle(self.side.end).y);
        self.for_each_bezier_mut(|b| b.rotate(angle_degree, center));
        self.for_each_line_mut(|l| l.rotate(angle_degree, center));
    }

    pub fn to(&mut self, dx: Cm, dy: Cm) {
        self.for_each_bezier_mut(|b| b.to(dx, dy));
        self.for_each_line_mut(|l| l.to(dx, dy));
    }
}
