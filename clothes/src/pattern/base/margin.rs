use super::base::Base;

pub struct Margin {
    pub base: Base,
}

impl Margin {
    pub fn new(base: &Base) -> Margin {
        let mut base = base.clone();
        let mut front = &mut base.front;
        front.center = front.center.parallel(1.0).right;
        front.arm_hole = (
            front.arm_hole.0.parallel(1.0).right,
            front.arm_hole.1.parallel(1.0).right,
        );
        front.neck = front.neck.parallel(1.0).right;
        front.shoulder = front.shoulder.parallel(1.0).right;
        front.side = front.side.parallel(1.0).right;
        front.waist = front.waist.parallel(1.0).right;
        front.waist.join(&mut front.side);
        front.chest_dart.fst = front.side.join_bezier(&mut front.arm_hole.0);
        front.chest_dart.snd = front.arm_hole.0.join(&front.arm_hole.1);
        front.side_dart.fst = front.arm_hole.1.join_line(&mut front.shoulder);
        front.shoulder.extend_end(1.0);
        front.side_dart.snd = front.shoulder.join_bezier(&mut front.neck);
        front.dart.fst = front.neck.join_line(&mut front.center);
        front.center.join(&mut front.waist);
        let mut back = &mut base.back;
        back.waist = back.waist.parallel(1.0).right;
        back.center = back.center.parallel(1.0).right;
        back.neck = back.neck.parallel(1.0).right;
        back.shoulder = back.shoulder.parallel(1.0).right;
        back.side = back.side.parallel(1.0).right;
        back.arm_hole = back.arm_hole.parallel(1.0).right;
        back.waist.join(&mut back.center);
        back.dart1.fst = back.center.join_bezier(&mut back.neck);
        back.dart1.snd = back.neck.join_line(&mut back.shoulder);
        back.dart2.fst = back.shoulder.join_bezier(&mut back.arm_hole);
        back.dart2.snd = back.arm_hole.join_line(&mut back.side);
        back.side.join(&mut back.waist);

        Margin { base }
    }
}
