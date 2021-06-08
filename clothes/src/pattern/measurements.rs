#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// centimeter
pub type Cm = f64;

/// look at [illustration](clothes/image/measurements-terminologies-illustration.jpg) for detail
/// #Examples
/// ```ignore
/// let measurements = Measurements {
///     ankle: 17.0,
///     waist: 60.0,
///     ..Default::default()
/// };
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Default)]
pub struct Measurements {
    pub ankle: Cm,
    pub armscye_depth: Cm,
    pub biceps: Cm,
    pub body_rise: Cm,
    pub bust: Cm,
    pub calf: Cm,
    pub crotch_depth: Cm,
    pub crotch_to_knee: Cm,
    pub elbow: Cm,
    pub head: Cm,
    pub high_ankle: Cm,
    pub hip: Cm,
    pub knee: Cm,
    pub lower_waist: Cm,
    pub max_thigh: Cm,
    pub nape_to_waist: Cm,
    pub neck_size: Cm,
    pub hps_to_waist: Cm,
    pub shoulder: Cm,
    pub sleeve_len: Cm,
    pub waist: Cm,
    pub waist_to_floor: Cm,
    pub waist_to_hip: Cm,
    pub waist_to_knee: Cm,
    pub wrist: Cm,
    pub x_back: Cm,
    pub x_front: Cm,
    pub x_shoulder: Cm,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Measurements {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Measurements {
        Measurements {
            ..Default::default()
        }
    }
}
