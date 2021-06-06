use pdf_canvas::{graphicsstate::Color, Canvas, Pdf};
use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape},
};

const A3_WIDTH: f32 = 842.0;
const A3_HEIGHT: f32 = 1190.0;
const PRECISION: u32 = 100;

pub fn create_pdf(drawing: Drawing) {
    let mut document = Pdf::create("drawing.pdf").expect("Create pdf file");

    // divide large drawings into multiple A3 pages
    let width = to_pt(drawing.width);
    let height = to_pt(drawing.height);
    // traverse vertically
    let mut i = 0.0;
    loop {
        let offset_y = i * A3_HEIGHT;
        // traverse horizontally
        let mut j = 0.0;
        loop {
            let offset_x = j * A3_WIDTH;
            document
                .render_page(A3_WIDTH, A3_HEIGHT, |canvas| {
                    canvas.set_stroke_color(Color::rgb(0, 0, 248))?;
                    for shape in drawing.clone().shapes {
                        draw_shape(canvas, shape, offset_x as f64, offset_y as f64);
                    }
                    canvas.stroke()
                })
                .expect("Write page");
            if offset_x + A3_WIDTH > width as f32 {
                break;
            }
            j += 1.0;
        }
        if offset_y + A3_HEIGHT > height as f32 {
            break;
        }
        i += 1.0;
    }
    document.finish().expect("Finish pdf document");
}

pub fn draw_shape(canvas: &mut Canvas, shape: Shape, offset_x: f64, offset_y: f64) {
    match shape {
        Shape::Bezier(mut b) => draw_bezier(canvas, b, offset_x, offset_y),
        Shape::Circle(mut c) => draw_circle(canvas, c, offset_x, offset_y),
        Shape::Line(mut l) => draw_line(canvas, l, offset_x, offset_y),
        Shape::Point(p) => draw_point(canvas, p, offset_x, offset_y),
    }
}

fn draw_bezier(canvas: &mut Canvas, b: Bezier, offset_x: f64, offset_y: f64) {
    let mut t = 0.0;
    let dt = 1.0 / PRECISION as f64;
    while t <= 1.0 {
        let mut p1 = b.point_at(t);
        let mut p2 = if t + dt < 1.0 {
            b.point_at(t + dt)
        } else {
            b.end()
        };
        p1 = to_pt_point(p1).to(-offset_x, -offset_y, 0.0);
        p2 = to_pt_point(p2).to(-offset_x, -offset_y, 0.0);
        canvas
            .line(p1.x as f32, p1.y as f32, p2.x as f32, p2.y as f32)
            .unwrap();
        t += dt;
    }
}

fn draw_circle(canvas: &mut Canvas, c: Circle, offset_x: f64, offset_y: f64) {
    let origin = to_pt_point(c.origin).to(-offset_x, -offset_y, 0.0);
    canvas
        .circle(origin.x as f32, origin.y as f32, to_pt(c.r) as f32)
        .unwrap();
}

fn draw_line(canvas: &mut Canvas, mut l: Line, offset_x: f64, offset_y: f64) {
    l.origin = to_pt_point(l.origin).to(-offset_x, -offset_y, 0.0);
    l.end = to_pt_point(l.end).to(-offset_x, -offset_y, 0.0);
    canvas
        .line(
            l.origin.x as f32,
            l.origin.y as f32,
            l.end.x as f32,
            l.end.y as f32,
        )
        .unwrap();
}

fn draw_point(canvas: &mut Canvas, p: Point, offset_x: f64, offset_y: f64) {
    let p = to_pt_point(p).to(-offset_x, -offset_y, 0.0);
    canvas
        .circle(p.x as f32, p.y as f32, to_pt(1.0) as f32)
        .unwrap();
}

fn to_pt(centimeter: f64) -> f64 {
    centimeter * 28.345175603955806
}

fn to_pt_point(p_centimeter: Point) -> Point {
    p_centimeter * 28.345175603955806
}

fn to_centimeter(point: f32) -> f64 {
    point as f64 / 28.345175603955806
}
