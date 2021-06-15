use pmdraw::shapes::{bezier::Bezier, point::Point};

use crate::pattern::{common::pattern::Pattern, measurements::Cm};

pub struct ShortSleeve {
    pub pattern: Pattern,
}

impl ShortSleeve {
    /// create short sleeve pattern
    /// - sleeve_length - from sholder to wrist
    /// - arm_hole_length - length of arm_hole of front and back
    pub fn new(sleeve_length: Cm, front_arm_hole: Cm, back_arm_hole: Cm) -> ShortSleeve {
        let mut pattern = Pattern::new();
        // let short sleeve length be 41.6 % of arm length (shoulder to wrist)
        let sleeve_len = sleeve_length * 0.416;
        let top_left = Point::new(0.0, 0.0);
        let arm_hole = front_arm_hole + back_arm_hole;
        let top_height = arm_hole / 6.0;
        let left_width = ((front_arm_hole - 0.5).powf(2.0) - top_height.powf(2.0)).sqrt();
        let right_width = ((back_arm_hole + 0.5).powf(2.0) - top_height.powf(2.0)).sqrt();
        let top_right = top_left.to(left_width + right_width, 0.0);
        let top_middle = top_left.middle(top_right).to(0.0, top_height);
        let top_left_middle = top_left.middle(top_middle).to_point(top_left, 2.5);
        let top_right_middle = top_right
            .line_to(top_middle)
            .parallel(1.9)
            .right
            .between(0.666666);
        let top_left_middle1 = top_middle
            .line_to(top_left)
            .parallel(1.0)
            .right
            .between(0.25);
        let top = Bezier::new_with_t(
            &vec![
                top_right,
                top_right_middle,
                top_middle,
                top_left_middle1,
                top_left_middle,
                top_left,
            ],
            &vec![0.0, 0.24, 0.38, 0.52, 0.76, 1.0],
        );
        let width_offset = (left_width + right_width) / 8.0 - 1.5;
        let bottom_origin = top_left.to(width_offset, -(sleeve_len - top_height));
        let bottom_end = top_right.to(-width_offset, -(sleeve_len - top_height));
        let bottom = bottom_origin.line_to(bottom_end);
        let right = Bezier::new(vec![
            bottom_end,
            bottom_end.to(-1.5, 0.0).middle(top_right),
            top_right,
        ]);
        let left = Bezier::new(vec![
            top_left,
            bottom_origin.to(1.5, 0.0).middle(top_left),
            bottom_origin,
        ]);
        // it's important to add them in counter clockwise to generate margin automatically
        pattern.add_line(bottom);
        pattern.add_curve(right);
        pattern.add_curve(top);
        pattern.add_curve(left);
        pattern.to(0.0, sleeve_len);
        pattern.generate_margin_diffrent_values(vec![1.0, 1.0, 1.6, 1.0]);
        ShortSleeve { pattern }
    }
}
