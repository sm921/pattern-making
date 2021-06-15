use pmdraw::shapes::{bezier::Bezier, point::Point};

use crate::pattern::{common::pattern::Pattern, measurements::Cm};

pub struct StandCollar {
    pub pattern: Pattern,
}

impl StandCollar {
    /// - placket_width - width of front placket
    /// - neck_hole - length of neck hole of front and back
    pub fn new(placket_width: Cm, neck_hole: Cm) -> StandCollar {
        let mut pattern = Pattern::new();
        let placket_width = placket_width / 2.0;
        let bottom_2nd_point = Point::new(placket_width, 0.3);
        let bottom_3rd_point = bottom_2nd_point.to(4.5, -0.3);
        let bottom_origin = bottom_2nd_point.to_point(bottom_3rd_point, -placket_width);
        let bottom_end = bottom_2nd_point.to(neck_hole, 0.7);
        let bottom_left = Bezier::new_with_t(
            &vec![
                bottom_origin,
                bottom_2nd_point,
                bottom_3rd_point,
                bottom_end,
            ],
            &vec![0.0, 0.19, 0.46, 1.0],
        );
        let top_origin = bottom_left.end().to(0.0, 4.0);
        let top_2nd_point = top_origin.to(-6.0, 0.0);
        let mut top_ctrl_line_end = bottom_left.origin().to_point(bottom_2nd_point, 3.0);
        top_ctrl_line_end.rotate(90.0, bottom_origin);
        let top_ctrl_line = top_2nd_point.line_to(top_ctrl_line_end);
        let top_3rd_point = top_ctrl_line.parallel(0.2).right.midddle();
        let top_end = top_ctrl_line.intersection(
            &bottom_origin
                .line_to(top_ctrl_line_end)
                .parallel(placket_width)
                .right,
        );
        let top_left = Bezier::new(vec![
            top_origin,
            top_2nd_point,
            top_3rd_point,
            top_end,
            top_ctrl_line_end.to_angular(-45.0, 0.9),
            bottom_origin,
        ]);
        let mirror_line = bottom_left.end().line_to(top_left.origin());
        let bottom_right = bottom_left.mirror(mirror_line);
        let top_right = top_left.mirror(mirror_line);
        pattern.add_curve(bottom_left);
        pattern.add_curve(bottom_right);
        pattern.add_curve(top_right);
        pattern.add_curve(top_left);
        pattern.generate_margin_except(0.5, vec![2]);
        StandCollar { pattern }
    }
}
