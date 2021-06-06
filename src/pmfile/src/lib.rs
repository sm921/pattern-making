use pdf_canvas::{graphicsstate::Color, Canvas, Pdf};
use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape},
};

const A3_WIDTH: f32 = 842.0;
const A3_HEIGHT: f32 = 1190.0;
const PRECISION: u32 = 100;

pub fn create_pdf(drawing: Drawing) {
    let mut document = Pdf::create("example.pdf").expect("Create pdf file");

    document
        .render_page(A3_WIDTH, A3_HEIGHT, |canvas| {
            canvas.set_stroke_color(Color::rgb(0, 0, 248))?;
            for shape in drawing.shapes {
                draw_shape(canvas, shape);
            }
            canvas.stroke()
        })
        .expect("Write page");
    document.finish().expect("Finish pdf document");
}

pub fn draw_shape(canvas: &mut Canvas, shape: Shape) {
    match shape {
        Shape::Bezier(b) => draw_bezier(canvas, b),
        Shape::Circle(c) => draw_circle(canvas, c),
        Shape::Line(l) => draw_line(canvas, l),
        Shape::Point(p) => draw_point(canvas, p),
    }
}

fn draw_bezier(canvas: &mut Canvas, b: Bezier) {
    let mut t = 0.0;
    let dt = 1.0 / PRECISION as f64;
    while t <= 1.0 {
        let p1 = b.point_at(t);
        let p2 = if t + dt < 1.0 {
            b.point_at(t + dt)
        } else {
            b.end()
        };
        canvas
            .line(
                convert_centimeter_to_point(p1.x),
                convert_centimeter_to_point(p1.y),
                convert_centimeter_to_point(p2.x),
                convert_centimeter_to_point(p2.y),
            )
            .unwrap();
        t += dt;
    }
}

fn draw_circle(canvas: &mut Canvas, c: Circle) {
    canvas
        .circle(
            convert_centimeter_to_point(c.origin.x),
            convert_centimeter_to_point(c.origin.y),
            convert_centimeter_to_point(c.r),
        )
        .unwrap();
}

fn draw_line(canvas: &mut Canvas, l: Line) {
    canvas
        .line(
            convert_centimeter_to_point(l.origin.x),
            convert_centimeter_to_point(l.origin.y),
            convert_centimeter_to_point(l.end.x),
            convert_centimeter_to_point(l.end.y),
        )
        .unwrap();
}

fn draw_point(canvas: &mut Canvas, p: Point) {
    canvas
        .circle(
            convert_centimeter_to_point(p.x),
            convert_centimeter_to_point(p.y),
            convert_centimeter_to_point(1.0),
        )
        .unwrap();
}

fn convert_centimeter_to_point(centimeter: f64) -> f32 {
    centimeter as f32 * 28.345175603955806
}
