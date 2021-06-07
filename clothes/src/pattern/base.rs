use std::f64::consts::PI;

use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line, point::Point},
};

use super::measurements::{Cm, Measurements};

pub struct Base {
    center_back_line: Line,
    center_front_line: Line,
    chest_line: Line,
    drawing_width: Cm,
    drawing_height: Cm,
    front_arm_hole: Bezier,
    front_neck: Bezier,
    shoulder_dart: Vec<Line>,
    shoulder_line: Line,
    waist_line: Line,
}

/// margin of the screen
const DRAWING_MARGIN: f64 = 9.0;

impl Base {
    pub fn draw(&self, window_width: u32, window_height: u32) {
        let mut draw = Drawing::new(self.drawing_width, self.drawing_height);
        for line in vec![
            self.center_back_line,
            self.center_front_line,
            self.chest_line,
            self.shoulder_dart[0],
            self.shoulder_dart[1],
            self.shoulder_line,
            self.waist_line,
        ] {
            draw.line(line)
        }
        for bezier in vec![&self.front_arm_hole, &self.front_neck] {
            draw.bezier(&bezier)
        }
        draw.show(window_width, window_height);
    }

    /// parameters are measurements of body and amount of dart
    pub fn new(m: Measurements<Cm>, dart: Cm) -> Base {
        Base::assert_measurements(&m);
        let drawing_width = m.waist / 2.0 + 2.0 + dart + DRAWING_MARGIN;
        let drawing_height = if m.hps_to_waist > m.nape_to_waist {
            m.hps_to_waist
        } else {
            m.nape_to_waist
        } + DRAWING_MARGIN;

        let waist = m.waist / 2.0 + 2.0 + dart;
        let neck_depth = m.neck_size / (2.0 * PI) * 1.7;

        let center_front_line = Line::new(
            Point::new(0.0, 0.0),
            Point::new(0.0, m.hps_to_waist - neck_depth - 0.5),
        );

        let front_neck = Bezier::new(vec![
            center_front_line.end,
            center_front_line
                .end
                .to(neck_depth / 3.0 * 2.0, neck_depth / 3.0),
            center_front_line.end.to(neck_depth, neck_depth),
        ]);

        let shoulder_line = Line::new(
            front_neck.end(),
            front_neck.end().to_angular(-22.0, m.shoulder),
        );

        let waist_line = Line::new(
            center_front_line.origin,
            center_front_line.origin.to(waist, 0.0),
        );

        let center_back_line = Line::new(waist_line.end, waist_line.end.to(0.0, m.nape_to_waist));

        let chest_line = Line::new(
            center_front_line.point_from_end((m.armscye_depth - neck_depth) + 3.0),
            center_back_line.point_from_end(m.armscye_depth + 3.0),
        );

        let h = chest_line.point_from_origin(m.x_front / 2.0 + 0.7 + 1.0);
        let shoulder_dart_middle = h.to(-0.7, 0.0).middle(chest_line.origin).to(0.7, 0.0);
        let y = (center_back_line.end - chest_line.end).norm() / 2.0;
        let shoulder_dart_end = h.to(0.0, y / 3.0 * 2.0);
        let shoulder_dart_bottom = Line::new(shoulder_dart_middle, shoulder_dart_end);
        let shoulder_dart_origin = Point::new(
            shoulder_dart_end.x - 0.7,
            shoulder_dart_middle.y
                + (shoulder_dart_bottom.len().powf(2.0)
                    - (h.x - 0.7 - shoulder_dart_middle.x).powf(2.0))
                .sqrt(),
        );
        let shoulder_dart = vec![
            Line::new(shoulder_dart_origin, shoulder_dart_middle),
            Line::new(shoulder_dart_middle, shoulder_dart_end),
        ];

        let front_arm_hole = Bezier::new(vec![
            shoulder_line.end,
            shoulder_line
                .end
                .middle(shoulder_dart_origin)
                .to_angular(135.0, 1.0),
            shoulder_dart_origin,
        ]);

        Base {
            center_back_line,
            center_front_line,
            chest_line,
            drawing_width,
            drawing_height,
            front_arm_hole,
            front_neck,
            shoulder_line,
            shoulder_dart,
            waist_line,
        }
    }

    fn assert_measurements(m: &Measurements<Cm>) {
        assert_ne!(m.waist, 0.0, "waist must be positive");
        assert_ne!(m.hps_to_waist, 0.0, "hps_to_waist must be positive");
        assert_ne!(m.nape_to_waist, 0.0, "nape_to_waist must be positive");
        assert_ne!(m.armscye_depth, 0.0, "armscye_depth must be positive");
        assert_ne!(m.shoulder, 0.0, "shoulder must be positive");
        assert_ne!(m.x_front, 0.0, "x_front must be positive");
    }
}
