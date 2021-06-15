use crate::pattern::{common::pattern::Pattern, measurements::Cm};

use super::stand_collar::StandCollar;

pub enum CollarType {
    Stand,
}

pub struct Collar {
    pub pattern: Pattern,
}

impl Collar {
    /// - stand color
    /// - neck_hole - length of neck hole (sum of front and back)
    /// - placket_width - width of placket of front
    pub fn new(collar_type: CollarType, neck_hole: Cm, placket_width: Cm) -> Collar {
        let pattern = match collar_type {
            CollarType::Stand => StandCollar::new(placket_width, neck_hole).pattern,
        };
        Collar { pattern }
    }
}
