use crate::pattern::{
    base::base::Base,
    common::pattern::Pattern,
    measurements::{Cm, Measurements},
};

use super::polo_body::PoloBody;

pub enum BodyType {
    Polo,
}
pub struct Body {
    pub patterns: Vec<Pattern>,
    pub placket_width: Cm,
    /// front.neck_hole + back.neck_hole
    pub neck: Cm,
    pub front_arm_hole: Cm,
    pub back_arm_hole: Cm,
}

impl Body {
    pub fn new(m: &Measurements, base: Base, body_type: BodyType, waist_margin: Cm) -> Body {
        let (patterns, placket_width, neck, front_arm_hole, back_arm_hole) = match body_type {
            BodyType::Polo => {
                let p = PoloBody::new(m, base, waist_margin);
                (
                    p.patterns,
                    p.placket_width,
                    p.neck,
                    p.front_arm_hole,
                    p.back_arm_hole,
                )
            }
        };
        Body {
            patterns,
            placket_width,
            neck,
            front_arm_hole,
            back_arm_hole,
        }
    }
}
