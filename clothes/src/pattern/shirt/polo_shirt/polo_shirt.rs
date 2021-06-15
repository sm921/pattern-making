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
        polo.shirt.collar.pattern.to(0.0, 78.0);
        polo.shirt.sleeve.pattern.to(0.0, 85.0);
        polo
    }
}
