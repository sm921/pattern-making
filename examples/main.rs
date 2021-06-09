use clothes::pattern::{base::Base, measurements::Measurements};
use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line, point::Point},
};
use pmfile::pdf::pdf;

fn main() {
    let mut drawing = Drawing::new(70.0, 60.0);
    drawing.line_from_point(0.0, 0.0, 30.0, 55.0);
    let p0 = Point::new(40.0, 50.0);
    let p1 = Point::new(24.0, 32.0);
    let l = Line::new(p0, p1);
    drawing.line(l);
    drawing.circle(Point::new(10.0, 10.0), 10.0);
    let o = Point::new(0.0, 18.0);
    let c1 = Point::new(20.0, 40.0);
    let c2 = Point::new(40.0, 12.0);
    let end = Point::new(60.0, 24.0);
    drawing.bezier_with_precision(&Bezier::new(vec![o, c1, c2, end]), 100);
    drawing.point(o);
    drawing.point(c1);
    drawing.point(c2);
    drawing.point(end);
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
    base.draw(900, 900);
    // drawing.show(900, 900);
    // pdf(&drawing);
}
