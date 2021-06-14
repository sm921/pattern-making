use std::f64::consts::PI;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use pmdraw::shapes::{bezier::Bezier, line::Line, point::Point};

use crate::pattern::{
    base::front::Front,
    common::dart::Dart,
    measurements::{Cm, Measurements},
};

use super::{back::Back, margin::Margin};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct Base {
    pub back: Back,
    pub front: Front,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Base {
    pub fn margin(&self) -> Margin {
        Margin::new(self)
    }
    pub fn for_each_bezier<T>(&self, mut callback: T)
    where
        T: FnMut(&Bezier) -> (),
    {
        for bezier in vec![
            &self.back.arm_hole,
            &self.back.neck,
            &self.front.arm_hole.0,
            &self.front.arm_hole.1,
            &self.front.neck,
        ] {
            callback(bezier)
        }
    }

    pub fn for_each_line<T>(&self, mut callback: T)
    where
        T: FnMut(Line) -> (),
    {
        for line in vec![
            self.back.dart1.fst,
            self.back.dart1.snd,
            self.back.dart2.fst,
            self.back.dart2.snd,
            self.back.center,
            self.back.center_dart,
            self.back.side,
            self.back.shoulder,
            self.back.shoulder_dart.fst,
            self.back.shoulder_dart.snd,
            self.back.waist,
            self.front.center,
            self.front.chest_dart.fst,
            self.front.chest_dart.snd,
            self.front.dart.fst,
            self.front.dart.snd,
            self.front.shoulder,
            self.front.side,
            self.front.side_dart.fst,
            self.front.side_dart.snd,
            self.front.waist,
        ] {
            callback(line)
        }
    }

    /// Create base pattern
    /// - m measurements of body
    /// - amount of dart - typically from 4 to 15 cm
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(m: Measurements, dart: Cm) -> Base {
        Base::assert_measurements(&m);
        let waist = m.waist / 2.0 + 2.0 + dart;
        let neck_depth = m.neck_size / 5.0 - 0.5;

        let center_front = Line::new(
            Point::new(0.0, m.hps_to_waist - neck_depth - 0.5),
            Point::new(0.0, 0.0),
        );

        let front_neck = Bezier::new(vec![
            center_front.origin.to(neck_depth, neck_depth),
            center_front
                .origin
                .to(neck_depth / 3.0 * 2.0, neck_depth / 3.0),
            center_front.origin,
        ]);

        let shoulder = Line::new(
            front_neck.origin().to_angular(-22.0, m.shoulder),
            front_neck.origin(),
        );

        let waist = Line::new(center_front.end, center_front.end.to(waist, 0.0));

        let center_back = Line::new(waist.end, waist.end.to(0.0, m.nape_to_waist));

        let chest = Line::new(
            center_front.point_from_origin((m.armscye_depth - neck_depth) + 3.0),
            center_back.point_from_end(m.armscye_depth + 3.0),
        );

        let arm_hole_left_bottom = chest.point_from_origin(m.x_front / 2.0 + 0.7 + 1.0);
        let chest_dart_middle = arm_hole_left_bottom
            .to(-0.7, 0.0)
            .middle(chest.origin)
            .to(0.7, 0.0);
        let arm_hole_radius = (center_back.end - chest.end).norm() / 2.0;
        let chest_dart_fst = arm_hole_left_bottom.to(0.0, arm_hole_radius / 3.0 * 2.0);
        let chest_dart_bottom = Line::new(chest_dart_middle, chest_dart_fst);
        let chest_dart_snd = Point::new(
            chest_dart_fst.x - 0.7,
            chest_dart_middle.y
                + (chest_dart_bottom.len().powf(2.0)
                    - (arm_hole_left_bottom.x - 0.7 - chest_dart_middle.x).powf(2.0))
                .sqrt(),
        );
        let chest_dart = Dart::new(chest_dart_fst, chest_dart_middle, chest_dart_snd);

        let back_neck_end = center_back
            .end
            .to(-neck_depth - 0.3, (neck_depth + 3.0) / 3.0 - 0.3);
        let back_neck = Bezier::new(vec![
            center_back.end,
            Point::new(
                back_neck_end.between(center_back.end, 0.333).x,
                center_back.end.y + 0.5,
            ),
            back_neck_end,
        ]);

        let shoulder_dart_width = (m.x_front + m.x_back) / 32.0;
        let back_shoulder = Line::new(
            back_neck_end,
            back_neck_end.to_angular(21.0, -(shoulder.len() + shoulder_dart_width)),
        );

        // create arm hole
        let front_arm_hole_1 = Bezier::new(vec![
            chest_dart_snd,
            shoulder
                .origin
                .middle(chest_dart_snd)
                .to_angular(135.0, 1.0),
            shoulder.origin,
        ]);
        let arm_hole_right_bottom = chest.point_from_end(m.x_back / 2.0);
        let arm_hole_right = Point::new(arm_hole_right_bottom.x, chest_dart_fst.y);
        let arm_hole_ctrl_radius = (arm_hole_right_bottom.x - arm_hole_left_bottom.x) / 6.0 + 0.5;
        let arm_hole_bottom = arm_hole_left_bottom.middle(arm_hole_right_bottom);
        let front_to_back_arm_hole = Bezier::new(vec![
            back_shoulder.end,
            arm_hole_right,
            arm_hole_right_bottom.to_angular(135.0, arm_hole_ctrl_radius),
            arm_hole_bottom,
            arm_hole_left_bottom.to_angular(45.0, arm_hole_ctrl_radius),
            chest_dart_fst,
        ]);
        let split_armhole = front_to_back_arm_hole.split(arm_hole_bottom);
        let back_arm_hole = split_armhole.fst;
        let front_arm_hole_0 = split_armhole.snd;
        let front_arm_hole = (front_arm_hole_0, front_arm_hole_1);

        // create dart
        let front_dart_middle = chest_dart.fst.at_x(chest.at_x(chest_dart_snd.x).x - 0.8);
        let front_dart = Dart::new(
            Point::new(front_dart_middle.x - dart * 0.08, 0.0),
            front_dart_middle,
            Point::new(front_dart_middle.x + dart * 0.08, 0.0),
        );

        let side_dart_middle = arm_hole_left_bottom.middle(arm_hole_right_bottom);
        let side_dart_right = Point::new(side_dart_middle.x, 0.0);
        let side = Line::new(side_dart_right, side_dart_middle);
        let side_back = Line::new(side_dart_middle, side_dart_right);
        let side_dart = Dart::new(
            Point::new(side_dart_middle.x - dart * 0.16, 0.0),
            side_dart_middle,
            side_dart_right,
        );

        let back_dart_1_middle = arm_hole_right_bottom.to(1.0, arm_hole_radius / 3.0);
        let back_dart_1 = Dart::new(
            waist.at_x(back_dart_1_middle.x).to(-dart * 0.18, 0.0),
            back_dart_1_middle,
            waist.at_x(back_dart_1_middle.x).to(dart * 0.18, 0.0),
        );

        let center_back_dart = Line::new(
            waist.point_from_end(dart * 0.08),
            center_back.end.middle(chest.end),
        );

        let shoulder_dart_middle =
            arm_hole_right_bottom.to(m.x_back / 4.0 + 0.5, arm_hole_radius + 1.5);
        let shoulder_dart_right = back_shoulder.point_from_origin(shoulder_dart_width * 0.9);
        let shoulder_dart_left =
            shoulder_dart_right.to_point(back_shoulder.end, shoulder_dart_width);
        let shoulder_dart = Dart::new(
            shoulder_dart_left,
            shoulder_dart_middle,
            shoulder_dart_right,
        );

        let back_dart_2_middle = chest
            .at_x(shoulder_dart_middle.to(-1.0, 0.0).x)
            .to(0.0, 2.5);
        let back_dart_2 = Dart::new(
            waist.at_x(back_dart_2_middle.x).to(-dart * 0.12, 0.0),
            back_dart_2_middle,
            waist.at_x(back_dart_2_middle.x).to(dart * 0.12, 0.0),
        );

        let split_wait = waist.split_at_x(side_dart_middle.x);
        let front_waist = split_wait.fst;
        let back_waist = split_wait.snd;

        let mut base = Base {
            back: Back {
                arm_hole: back_arm_hole,
                dart1: back_dart_1,
                dart2: back_dart_2,
                neck: back_neck,
                side: side_back,
                shoulder: back_shoulder,
                shoulder_dart,
                center: center_back,
                center_dart: center_back_dart,
                waist: back_waist,
            },
            front: Front {
                center: center_front,
                arm_hole: front_arm_hole,
                dart: front_dart,
                neck: front_neck,
                shoulder,
                side,
                side_dart,
                chest_dart,
                waist: front_waist,
            },
        };
        base.back.to(3.0, 1.0);
        base.front.to(1.0, 1.0);
        base
    }

    fn assert_measurements(m: &Measurements) {
        assert_ne!(m.waist, 0.0, "waist must be positive");
        assert_ne!(m.hps_to_waist, 0.0, "hps_to_waist must be positive");
        assert_ne!(m.nape_to_waist, 0.0, "nape_to_waist must be positive");
        assert_ne!(m.armscye_depth, 0.0, "armscye_depth must be positive");
        assert_ne!(m.shoulder, 0.0, "shoulder must be positive");
        assert_ne!(m.x_front, 0.0, "x_front must be positive");
        assert_ne!(m.x_back, 0.0, "x_front must be positive");
    }
}
