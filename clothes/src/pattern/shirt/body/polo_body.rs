use pmdraw::shapes::bezier::Bezier;

use crate::pattern::{
    base::base::Base,
    common::pattern::Pattern,
    measurements::{Cm, Measurements},
};

pub struct PoloBody {
    pub placket_width: Cm,
    pub patterns: Vec<Pattern>,
    pub neck: Cm,
    pub front_arm_hole: Cm,
    pub back_arm_hole: Cm,
}

impl PoloBody {
    /// create polo front pattern
    pub fn new(measurements: &Measurements, base: Base, waist_margin: Cm) -> PoloBody {
        let mut base = base.clone();
        let mut front = Pattern::new();
        let f_base = base.front;
        let waist = f_base.waist;
        let placket_width = 3.4;
        let waist = waist.origin.line_to(waist.point_from_end(1.0));
        let bottom = waist.parallel(measurements.waist_to_hip).right;
        let arm_hole_origin = f_base.arm_hole.0.range.from.to(0.0, -3.0);
        let side = Bezier::new(vec![bottom.end, waist.end, arm_hole_origin]);
        let arm_hole_end = f_base.shoulder.parallel(1.0).right.point_from_origin(-2.5);
        let front_arm_hole = Bezier::new(vec![
            arm_hole_origin,
            f_base.arm_hole.0.point_at(0.9),
            arm_hole_end,
        ]);
        let shoulder = arm_hole_end.line_to(f_base.shoulder.point_from_end(-0.8));
        let mut front_neck = f_base.neck.clone();
        let neck_end = front_neck.end().to(0.0, -1.0);
        let neck_end_index = front_neck.get_range_index().1;
        let center = neck_end.line_to(bottom.origin);
        front_neck.refit(
            |fit_points| {
                fit_points[0] = shoulder.end;
                fit_points[neck_end_index] = neck_end;
                fit_points.to_vec()
            },
            Some(shoulder.end),
            Some(neck_end),
        );
        let placket_end = f_base.arm_hole.0.range.from.to(-waist.len() - 1.0, 0.0);
        let placket_end_mark = placket_end.line_to(placket_end.to(1.0, 0.0));
        let placket_cut_mark = placket_end_mark.parallel(3.0).left;
        let placket_length = (center.origin - placket_end).y;
        let button2 = placket_end_mark.parallel(placket_length * 0.36).left;
        let button1 = placket_end_mark.parallel(placket_length * 0.72).left;
        front.add_line(bottom);
        front.add_curve(side);
        front.add_curve(front_arm_hole.clone());
        front.add_line(shoulder);
        front.add_curve(front_neck.clone());
        front.add_line(center);
        front.generate_margin_diffrent_values(vec![1.0, 1.0, 0.5, 1.0, 0.5, 0.0]);
        front.add_line(placket_end_mark);
        front.add_line(placket_cut_mark);
        front.add_line(button2);
        front.add_line(button1);
        front.to(0.0, measurements.waist_to_hip);

        let mut placket = Pattern::new();
        let bottom = placket_end.line_to(placket_end.to(placket_width + placket_width + 0.1, 0.0));
        let right = bottom.end.line_to(bottom.end.to(0.0, placket_length - 3.0));
        let right_middle = right.end.line_to(right.end.to(-placket_width - 0.1, 0.0));
        let right_top = right_middle.end.line_to(right_middle.end.to(0.0, 3.0));
        let top = right_top.end.line_to(right_top.end.to(-placket_width, 0.0));
        let left = top.end.line_to(bottom.origin);
        let center_mark_origin = bottom.origin.to(placket_width, 0.0);
        let center_mark = center_mark_origin.line_to(center_mark_origin.to(0.0, placket_length));
        placket.add_line(bottom);
        placket.add_line(right);
        placket.add_line(right_middle);
        placket.add_line(right_top);
        placket.add_line(top);
        placket.add_line(left);
        placket.generate_margin_diffrent_values(vec![1.0, 0.8, 0.0, 1.25, 0.8, 0.8]);
        placket.add_line(center_mark);
        placket.to(60.0, measurements.waist_to_hip);

        let mut placket2 = Pattern::new();
        let top = right
            .end
            .line_to(right.end.to(-placket_width - placket_width - 0.1, 0.0));
        let left = top.end.line_to(bottom.origin);
        placket2.add_line(bottom);
        placket2.add_line(right);
        placket2.add_line(top);
        placket2.add_line(left);
        placket2.generate_margin_diffrent_values(vec![1.0, 0.5, 0.0, 0.5]);
        let center_mark =
            center_mark_origin.line_to(center_mark_origin.to(0.0, placket_length - 3.0));
        placket2.add_line(center_mark);
        placket2.to(70.0, measurements.waist_to_hip);

        let mut back = Pattern::new();
        let mut b_base = &mut base.back;
        b_base.waist.origin = b_base.waist.point_from_origin(1.0);
        let bottom = b_base.waist.parallel(measurements.waist_to_hip).right;
        let center = bottom.end.line_to(b_base.center.end);
        let back_neck = b_base.neck.clone();
        let mut shoulder = b_base.shoulder;
        shoulder.end.rotate(
            -b_base.shoulder_dart.fst.angle() + b_base.shoulder_dart.snd.angle(),
            b_base.shoulder_dart.middle(),
        );
        shoulder.extend_end(2.5);
        let arm_hole_end = b_base.arm_hole.range.to.to(0.0, -3.0);
        let mut back_arm_hole = b_base.arm_hole.clone();
        let arm_hole_end_index = back_arm_hole.get_range_index().1;
        back_arm_hole.refit(
            |fit_points| {
                fit_points[0] = shoulder.end;
                fit_points[arm_hole_end_index] = arm_hole_end;
                fit_points.to_vec()
            },
            Some(shoulder.end),
            Some(arm_hole_end),
        );
        let side = Bezier::new(vec![arm_hole_end, b_base.waist.origin, bottom.origin]);
        back.add_line(bottom);
        back.add_line(center);
        back.add_curve(back_neck.clone());
        back.add_line(shoulder);
        back.add_curve(back_arm_hole.clone());
        back.add_curve(side);
        back.generate_margin_diffrent_values(vec![1.0, 0.0, 0.5, 1.0, 0.5, 1.0]);
        back.to(6.0, measurements.waist_to_hip);
        PoloBody {
            patterns: vec![front, back, placket, placket2],
            placket_width,
            neck: front_neck.len() + back_neck.len(),
            front_arm_hole: front_arm_hole.len(),
            back_arm_hole: back_arm_hole.len(),
        }
    }
}
