use clothes::pattern::{measurements::Measurements, shirt::polo_shirt::polo_shirt::PoloShirt};
fn main() {
    let measurements = Measurements {
        armscye_depth: 22.0,
        hps_to_waist: 47.0,
        nape_to_waist: 47.0,
        neck_size: 40.5,
        shoulder: 15.5,
        sleeve_len: 59.0,
        waist: 70.0,
        waist_to_hip: 19.0,
        wrist: 17.3,
        x_front: 36.0,
        x_back: 36.0,
        ..Default::default()
    };
    let waist_margin = 9.5;
    // let base = Base::new(&measurements, waist_margin);
    // let draw = export_base(&base, None, None, true);
    // draw.show(900, 900);
    let polo_shirt = PoloShirt::new(
        &measurements,
        waist_margin,
        clothes::pattern::shirt::collar::collar::CollarType::Stand,
    );
    polo_shirt.shirt.export_to_pdf("polo.pdf", 70.0, 140.0);
    polo_shirt.shirt.show(70.0, 140.0);
}
