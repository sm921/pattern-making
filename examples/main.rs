use clothes::{
    export::pdf::export_base,
    pattern::{base::base::Base, measurements::Measurements},
};
fn main() {
    let mut base = Base::new(
        Measurements {
            waist: 60.0,
            hps_to_waist: 57.0,
            nape_to_waist: 57.0,
            armscye_depth: 21.0,
            neck_size: 27.0,
            shoulder: 13.0,
            x_front: 27.0,
            x_back: 28.0,
            ..Default::default()
        },
        15.0,
    );
    base.back.to(1.0, 1.0);
    export_base(&base, 900, 900, Some(560.0), Some(690.0));
}
