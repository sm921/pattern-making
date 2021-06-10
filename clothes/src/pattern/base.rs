use std::f64::consts::PI;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line, point::Point},
};

use super::measurements::{Cm, Measurements};
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Base {
    back: Back,
    front: Front,
}

struct Back {
    arm_hole: Bezier,
    dart1: (Line, Line),
    dart2: (Line, Line),
    neck: Bezier,
    side: Line,
    shoulder: Line,
    center: Line,
    center_dart: Line,
    waist: Line,
}

struct Front {
    center: Line,
    arm_hole: Bezier,
    dart: (Line, Line),
    neck: Bezier,
    chest_dart: (Line, Line),
    shoulder: Line,
    shoulder_dart: (Line, Line),
    side: Line,
    side_dart: (Line, Line),
    waist: Line,
}

/// margin of the screen
const DRAWING_MARGIN: f64 = 9.0;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Base {
    pub fn draw(&self, window_width: u32, window_height: u32) -> Drawing {
        let drawing_width = self.back.waist.len() + DRAWING_MARGIN;
        let drawing_height = self.front.shoulder.origin.y + DRAWING_MARGIN;

        let mut draw = Drawing::new(drawing_width, drawing_height);
        for line in vec![
            self.back.shoulder,
            self.back.dart1.0,
            self.back.dart1.1,
            self.back.dart2.0,
            self.back.dart2.1,
            self.back.center,
            self.back.center_dart,
            self.back.side,
            self.front.center,
            self.front.chest_dart.0,
            self.front.chest_dart.1,
            self.front.dart.0,
            self.front.dart.1,
            self.front.shoulder,
            self.front.shoulder_dart.0,
            self.front.shoulder_dart.1,
            self.front.side,
            self.front.side_dart.0,
            self.front.side_dart.1,
            self.front.waist,
        ] {
            draw.line(line)
        }

        for bezier in vec![
            &self.back.arm_hole,
            &self.back.neck,
            &self.front.arm_hole,
            &self.front.neck,
        ] {
            draw.bezier(&bezier)
        }
        // draw.show(window_width, window_height);
        draw
    }

    /// Create base pattern
    /// - m measurements of body
    /// - amount of dart - typically from 4 to 15 cm
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(m: Measurements, dart: Cm) -> Base {
        Base::assert_measurements(&m);
        let waist = m.waist / 2.0 + 2.0 + dart;
        let neck_depth = m.neck_size / (2.0 * PI) * 1.7;

        let center_front = Line::new(
            Point::new(0.0, 0.0),
            Point::new(0.0, m.hps_to_waist - neck_depth - 0.5),
        );

        let front_neck = Bezier::new(vec![
            center_front.end,
            center_front
                .end
                .to(neck_depth / 3.0 * 2.0, neck_depth / 3.0),
            center_front.end.to(neck_depth, neck_depth),
        ]);

        let shoulder = Line::new(
            front_neck.end(),
            front_neck.end().to_angular(-22.0, m.shoulder),
        );

        let waist = Line::new(center_front.origin, center_front.origin.to(waist, 0.0));

        let center_back = Line::new(waist.end, waist.end.to(0.0, m.nape_to_waist));

        let chest = Line::new(
            center_front.point_from_end((m.armscye_depth - neck_depth) + 3.0),
            center_back.point_from_end(m.armscye_depth + 3.0),
        );

        let arm_hole_left_bottom = chest.point_from_origin(m.x_front / 2.0 + 0.7 + 1.0);
        let chest_dart_middle = arm_hole_left_bottom
            .to(-0.7, 0.0)
            .middle(chest.origin)
            .to(0.7, 0.0);
        let arm_hole_radius = (center_back.end - chest.end).norm() / 2.0;
        let chest_dart_end = arm_hole_left_bottom.to(0.0, arm_hole_radius / 3.0 * 2.0);
        let chest_dart_bottom = Line::new(chest_dart_middle, chest_dart_end);
        let chest_dart_origin = Point::new(
            chest_dart_end.x - 0.7,
            chest_dart_middle.y
                + (chest_dart_bottom.len().powf(2.0)
                    - (arm_hole_left_bottom.x - 0.7 - chest_dart_middle.x).powf(2.0))
                .sqrt(),
        );
        let chest_dart = (
            Line::new(chest_dart_origin, chest_dart_middle),
            Line::new(chest_dart_middle, chest_dart_end),
        );

        let back_neck_origin = center_back
            .end
            .to(-neck_depth - 0.3, (neck_depth + 3.0) / 3.0 - 0.3);
        let back_neck = Bezier::new(vec![
            back_neck_origin,
            Point::new(
                back_neck_origin.between(center_back.end, 0.333).x,
                center_back.end.y + 0.5,
            ),
            center_back.end,
        ]);

        let shoulder_dart_width = (m.x_front + m.x_back) / 32.0;
        let back_shoulder = Line::new(
            back_neck_origin,
            back_neck_origin.to_angular(21.0, -(shoulder.len() + shoulder_dart_width)),
        );

        // create arm hole
        let front_arm_hole = Bezier::new(vec![
            shoulder.end,
            shoulder
                .end
                .middle(chest_dart_origin)
                .to_angular(135.0, 1.0),
            chest_dart_origin,
        ]);
        let arm_hole_right_bottom = chest.point_from_end(m.x_back / 2.0);
        let arm_hole_right = Point::new(arm_hole_right_bottom.x, chest_dart_end.y);
        let arm_hole_ctrl_radius = (arm_hole_right_bottom.x - arm_hole_left_bottom.x) / 6.0 + 0.5;
        let mut back_arm_hole = Bezier::new(vec![
            chest_dart_end,
            arm_hole_left_bottom.to_angular(45.0, arm_hole_ctrl_radius),
            arm_hole_left_bottom.middle(arm_hole_right_bottom),
            arm_hole_right_bottom.to_angular(135.0, arm_hole_ctrl_radius),
            arm_hole_right,
            back_shoulder.end,
        ]);
        back_arm_hole.set_range(chest_dart_end, arm_hole_right);

        // create dart
        let front_dart_middle = chest_dart.1.at_x(chest.at_x(chest_dart_origin.x).x - 0.8);
        let front_dart_left = Line::new(
            front_dart_middle,
            Point::new(front_dart_middle.x - dart * 0.08, 0.0),
        );
        let mut front_dart_right = front_dart_left.clone();
        front_dart_right.end = front_dart_right.end.to(dart * 0.08 * 2.0, 0.0);
        let front_dart = (front_dart_left, front_dart_right);

        let side_dart_middle = arm_hole_left_bottom.middle(arm_hole_right_bottom);
        let side_dart_right = waist.at_x(side_dart_middle.x);
        let side = Line::new(side_dart_middle, side_dart_right);
        let side_dart = (
            Line::new(side_dart_middle, side_dart_right.to(-dart * 0.16, 0.0)),
            Line::new(side_dart_middle, side_dart_right),
        );

        let back_dart_1_middle = arm_hole_right_bottom.to(1.0, arm_hole_radius / 3.0);
        let back_dart_1_left = waist.at_x(back_dart_1_middle.x).to(-dart * 0.18, 0.0);
        let back_dart_1_right = waist.at_x(back_dart_1_middle.x).to(dart * 0.18, 0.0);
        let back_dart_1 = (
            Line::new(back_dart_1_middle, back_dart_1_left),
            Line::new(back_dart_1_middle, back_dart_1_right),
        );

        let center_back_dart = Line::new(
            center_back.end.middle(chest.end),
            waist.point_from_end(dart * 0.08),
        );

        let shoulder_dart_middle =
            arm_hole_right_bottom.to(m.x_back / 4.0 + 0.5, arm_hole_radius + 1.5);
        let shoulder_dart_right = back_shoulder.point_from_origin(shoulder_dart_width * 0.9);
        let shoulder_dart_left =
            shoulder_dart_right.to_point(back_shoulder.end, shoulder_dart_width);
        let shoulder_dart = (
            Line::new(shoulder_dart_middle, shoulder_dart_left),
            Line::new(shoulder_dart_middle, shoulder_dart_right),
        );

        let back_dart_2_middle = chest
            .at_x(shoulder_dart_middle.to(-1.0, 0.0).x)
            .to(0.0, 2.5);
        let back_dart_2_left = waist.at_x(back_dart_2_middle.x).to(-dart * 0.12, 0.0);
        let back_dart_2_right = waist.at_x(back_dart_2_middle.x).to(dart * 0.12, 0.0);
        let back_dart_2 = (
            Line::new(back_dart_2_middle, back_dart_2_left),
            Line::new(back_dart_2_middle, back_dart_2_right),
        );

        let (front_waist, back_waist) = waist.split_at_x(side_dart_middle.x);

        Base {
            back: Back {
                arm_hole: back_arm_hole,
                dart1: back_dart_1,
                dart2: back_dart_2,
                neck: back_neck,
                side,
                shoulder: back_shoulder,
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
                shoulder_dart,
                side,
                side_dart,
                chest_dart,
                waist: front_waist,
            },
        }
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
