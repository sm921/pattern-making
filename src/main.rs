use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line, point::Point},
};
use pmfile::create_pdf;

fn main() {
    create_pdf();
    let mut drawing = Drawing::new();
    drawing.draw_line(0.0, 0.0, 30.0, 55.0);
    let p0 = Point::new(40.0, 50.0);
    let p1 = Point::new(24.0, 32.0);
    let l = Line::new(p0, p1);
    drawing.draw_line_struct(l);
    drawing.draw_circle(Point::new(10.0, 10.0), 10.0);
    let o = Point::new(0.0, 18.0);
    let c1 = Point::new(20.0, 40.0);
    let c2 = Point::new(40.0, 12.0);
    let end = Point::new(60.0, 24.0);
    drawing.draw_bezier(Bezier::new(vec![o, c1, /*c2,*/ end]), 100);
    drawing.draw_point(o);
    drawing.draw_point(c1);
    drawing.draw_point(c2);
    drawing.draw_point(end);
    drawing.show(900, 900, 0.0..70.0, 0.0..100.0);
}
