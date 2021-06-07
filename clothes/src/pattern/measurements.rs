/// centimeter
pub type Cm = f64;
pub type Inch = f64;

/// look at clothes/image/measurements-terminologies-illustration.jpg for detail
/// #Examples
/// ```ignore
/// let measurements = Measurements {
///     ankle: 17.0,
///     waist: 60.0,
///     ..Default::default()
/// };
/// ```
#[derive(Default)]
pub struct Measurements<Unit> {
    pub ankle: Unit,
    pub armscye_depth: Unit,
    pub biceps: Unit,
    pub body_rise: Unit,
    pub bust: Unit,
    pub calf: Unit,
    pub crotch_depth: Unit,
    pub crotch_to_knee: Unit,
    pub elbow: Unit,
    pub head: Unit,
    pub high_ankle: Unit,
    pub hip: Unit,
    pub knee: Unit,
    pub lower_waist: Unit,
    pub max_thigh: Unit,
    pub nape_to_waist: Unit,
    pub neck_size: Unit,
    pub hps_to_waist: Unit,
    pub shoulder: Unit,
    pub sleeve_len: Unit,
    pub waist: Unit,
    pub waist_to_floor: Unit,
    pub waist_to_hip: Unit,
    pub waist_to_knee: Unit,
    pub wrist: Unit,
    pub x_back: Unit,
    pub x_front: Unit,
    pub x_shoulder: Unit,
}
