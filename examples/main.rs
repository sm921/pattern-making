use clothes::{
    export::pdf::export_base,
    pattern::{base::base::Base, measurements::Measurements},
};
fn main() {
    let base = Base::new(
        Measurements {
            waist: 70.0,
            hps_to_waist: 47.0,
            nape_to_waist: 47.0,
            armscye_depth: 22.0,
            neck_size: 40.5,
            shoulder: 15.5,
            x_front: 36.0,
            x_back: 36.0,
            ..Default::default()
        },
        9.5,
    );
    let draw = export_base(&base, None, None, true);
    draw.show(900, 900);
}
