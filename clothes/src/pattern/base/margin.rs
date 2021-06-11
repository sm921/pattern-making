use super::base::Base;

pub struct Margin {
    pub base: Base,
}

impl Margin {
    pub fn new(base: &Base) -> Margin {
        let mut base = base.clone();
        base.front.center = base.front.center.parallel(1.0).right;
        base.front.arm_hole = (
            base.front.arm_hole.0.parallel(1.0).right,
            base.front.arm_hole.1.parallel(1.0).right,
        );
        base.front.neck = base.front.neck.parallel(1.0).right;
        base.front.shoulder = base.front.shoulder.parallel(1.0).right;
        base.front.side = base.front.side.parallel(1.0).right;
        base.front.waist = base.front.waist.parallel(1.0).right;
        base.front.waist.join(&mut base.front.side);
        base.front.waist.join(&mut base.front.center);

        Margin { base }
    }
}
