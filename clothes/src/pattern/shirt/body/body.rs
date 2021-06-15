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
}

impl Body {
    pub fn new(m: &Measurements, base: Base, body_type: BodyType, waist_margin: Cm) -> Body {
        let patterns = match body_type {
            BodyType::Polo => PoloBody::new(m, base, waist_margin).patterns,
        };
        Body { patterns }
    }
}
