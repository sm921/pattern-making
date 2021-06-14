use clothes::{
    export::pdf::export_base,
    pattern::{base::base::Base, measurements::Measurements},
};
fn main() {
    let base = Base::new(
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
    let draw = export_base(&base, None, None, true);
    draw.show(900, 900);
}
