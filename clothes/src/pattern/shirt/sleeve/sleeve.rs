use crate::pattern::{common::pattern::Pattern, measurements::Cm};

use super::short_sleeve::ShortSleeve;

pub enum SleeveType {
    Short,
}

pub struct Sleeve {
    pub pattern: Pattern,
}

impl Sleeve {
    pub fn new(
        sleeve_type: SleeveType,
        sleeve_length: Cm,
        front_arm_hole: Cm,
        back_arm_hole: Cm,
    ) -> Sleeve {
        let pattern = match sleeve_type {
            SleeveType::Short => {
                ShortSleeve::new(sleeve_length, front_arm_hole, back_arm_hole).pattern
            }
        };
        Sleeve { pattern }
    }
}
