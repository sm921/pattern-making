use crate::pattern::{
    measurements::{Cm, Measurements},
    shirt::{
        body::body::BodyType, collar::collar::CollarType, shirt::Shirt, sleeve::sleeve::SleeveType,
    },
};

/// short sleeve shirts
pub struct PoloShirt {
    pub shirt: Shirt,
}

impl PoloShirt {
    /// Create short sleeve shirts pattern
    pub fn new(m: &Measurements, waist_margin: Cm, collar_type: CollarType) -> PoloShirt {
        let shirt = Shirt::new(
            m,
            collar_type,
            BodyType::Polo,
            SleeveType::Short,
            waist_margin,
        );
        let mut polo = PoloShirt { shirt };
        polo.shirt.body.patterns[0].to(1.0, 6.0);
        polo.shirt.body.patterns[1].to(2.0, 6.0);
        polo.shirt.body.patterns[2].to(1.0, 5.0);
        polo.shirt.body.patterns[3].to(1.0, 5.0);
        polo.shirt.collar.pattern.to(5.0, 87.0);
        polo.shirt.sleeve.pattern.to(5.0, 89.0);
        polo
    }
}
