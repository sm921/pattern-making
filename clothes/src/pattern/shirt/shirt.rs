use pmdraw::drawing::Drawing;
use pmfile::pdf2::pdf2;

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
        let collar = Collar::new(collar_type, body.neck, body.placket_width);
        let sleeve = Sleeve::new(
            sleeve_type,
            m.sleeve_len,
            body.front_arm_hole,
            body.back_arm_hole,
        );
        Shirt {
            body,
            collar,
            sleeve,
        }
    }

    pub fn draw(&self, width: Cm, height: Cm) -> Drawing {
        let mut drawing = Drawing::new(width, height);
        for pattern in &self.body.patterns {
            pattern.draw(&mut drawing)
        }
        self.collar.pattern.draw(&mut drawing);
        self.sleeve.pattern.draw(&mut drawing);
        drawing
    }

    pub fn export_to_pdf(&self, file_name: &str, width: Cm, height: Cm) {
        pdf2(file_name, &self.draw(width, height), None, None);
    }

    pub fn show(&self, width: Cm, height: Cm) {
        self.draw(width, height).show(900, 900);
    }
}
