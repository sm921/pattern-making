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
        let bottom_ctrl_point = Point::new(placket_width, 0.3);
        let bottom_middle = bottom_ctrl_point.to(4.5, -0.3);
        let bottom_origin = bottom_ctrl_point.to_point(bottom_middle, -placket_width);
        let bottom_end = bottom_ctrl_point.to(neck_hole, 0.7);
        let mirror_line = bottom_end.line_to(bottom_end.to(0.0, 10.0));
        let bottom = Bezier::new(vec![
            bottom_origin,
            bottom_middle,
            bottom_end,
            bottom_middle.mirror(mirror_line),
            bottom_origin.mirror(mirror_line),
        ]);
        let top_origin = bottom_end.to(0.0, 4.0);
        let mut top_ctrl_line_end = bottom_origin.to_point(bottom_ctrl_point, 3.0);
        top_ctrl_line_end.rotate(90.0, bottom_origin);
        let top_ctrl_line = top_origin.to(-6.0, 0.0).line_to(top_ctrl_line_end);
        let top_3rd_point = top_ctrl_line.parallel(0.2).right.midddle();
        let top_end = top_ctrl_line.intersection(
            &bottom_origin
                .line_to(top_ctrl_line_end)
                .parallel(placket_width)
                .right,
        );
        let top = Bezier::new_with_t(
            &vec![
                bottom_origin.mirror(mirror_line),
                top_end.mirror(mirror_line),
                top_3rd_point.mirror(mirror_line),
                top_origin,
                top_3rd_point,
                top_end,
                bottom_origin,
            ],
            &vec![0.0, 0.16, 0.32, 0.5, 0.68, 0.84, 1.0],
        );
        pattern.add_curve(bottom);
        pattern.add_curve(top);
        pattern.generate_margin(0.5);
        StandCollar { pattern }
    }
}
