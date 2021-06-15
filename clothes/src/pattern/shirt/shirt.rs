use pmdraw::drawing::Drawing;

use crate::pattern::{
    base::base::Base,
    measurements::{Cm, Measurements},
    shirt::collar::collar::{Collar, CollarType},
};

use super::{
    body::body::{Body, BodyType},
    sleeve::sleeve::{Sleeve, SleeveType},
};

/// short sleeve shirts
pub struct Shirt {
    pub body: Body,
    pub sleeve: Sleeve,
    pub collar: Collar,
}

impl Shirt {
    /// Create a shirt pattern
    pub fn new(
        m: &Measurements,
        collar_type: CollarType,
        body_type: BodyType,
        sleeve_type: SleeveType,
        waist_margin: Cm,
    ) -> Shirt {
        let base = Base::new(m, waist_margin);
        let body = Body::new(m, base, body_type, waist_margin);
        // todo calculate actual length from front's placket
        let placket_width = 6.0;
        // todo front.neck_hole + back.neck_hole
        let neck_hole = 21.7;
        let collar = Collar::new(collar_type, neck_hole, placket_width);
        // todo calculate actual length from front and base
        let front_arm_hole = 21.0;
        let back_arm_hole = 20.0;
        let sleeve = Sleeve::new(sleeve_type, m.sleeve_len, front_arm_hole, back_arm_hole);
        Shirt {
            body,
            collar,
            sleeve,
        }
    }

    pub fn show(&self) {
        let mut drawing = Drawing::new(90.0, 150.0);
        for pattern in &self.body.patterns {
            pattern.draw(&mut drawing)
        }
        self.collar.pattern.draw(&mut drawing);
        self.sleeve.pattern.draw(&mut drawing);
        drawing.show(900, 900);
    }
}
