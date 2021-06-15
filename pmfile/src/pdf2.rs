use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{Canvas, Pdf};

use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line, point::Point, Shape},
};

// width in pdf point
const A3_WIDTH: f32 = 842.0;
// height in pdf point
const A3_HEIGHT: f32 = 1190.0;
const PRECISION: u32 = 100;

/// Create PDF file
/// - paper_width - width of document in millimeter
/// - paper_width - width of document in millimeter
pub fn pdf2(
    file_name: &str,
    drawing: &Drawing,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
) -> () {
    let mut file_path = String::from("clothes/out/");
    file_path.push_str(file_name);
    let mut document = Pdf::create(&file_path).expect("Create pdf file");

    let paper_width = match paper_width {
        Some(custom_width) => to_pt(custom_width / 10.0),
        None => A3_WIDTH,
    };
    let paper_height = match paper_height {
        Some(custom_height) => to_pt(custom_height / 10.0),
        None => A3_HEIGHT,
    };

    let width = to_pt(drawing.width);
    let height = to_pt(drawing.height);
    // traverse vertically
    let mut i = 0.0;
    loop {
        let offset_y = i * paper_height;
        // traverse horizontally
        let mut j = 0.0;
        loop {
            let offset_x = j * paper_width;
            document
                .render_page(paper_width, paper_height, |canvas| {
                    canvas.set_stroke_color(Color::rgb(0, 0, 248))?;
                    for shape in drawing.shapes.clone() {
                        match shape {
                            Shape::Bezier(b) => draw_bezier(canvas, b, offset_x, offset_y),
                            Shape::Line(l) => draw_line(canvas, l, offset_x, offset_y),
                            _ => todo!(),
                        }
                    }
                    canvas.stroke()
                })
                .expect("Write page");
            if offset_x + paper_width > width {
                break;
            }
            j += 1.0;
        }
        if offset_y + paper_height > height {
            break;
        }
        i += 1.0;
    }
    document.finish().expect("Finish pdf document");
}

fn draw_bezier(canvas: &mut Canvas, b: Bezier, offset_x: f32, offset_y: f32) {
    let t_range = b.t_range();
    let mut t = t_range.from;
    let dt = t_range.to / PRECISION as f64;
    while t <= t_range.to {
        let p1 = b.point_at(t);
        let p2 = if t + dt < t_range.to {
            b.point_at(t + dt)
        } else {
            b.point_at(t_range.to)
        };
        draw_line(canvas, Line::new(p1, p2), offset_x, offset_y);
        t += dt;
    }
}

fn draw_line(canvas: &mut Canvas, mut l: Line, offset_x: f32, offset_y: f32) {
    l.origin = to_pt_point(l.origin).to(-offset_x as f64, -offset_y as f64);
    l.end = to_pt_point(l.end).to(-offset_x as f64, -offset_y as f64);
    canvas
        .line(
            l.origin.x as f32,
            l.origin.y as f32,
            l.end.x as f32,
            l.end.y as f32,
        )
        .unwrap();
}

fn to_pt(centimeter: f64) -> f32 {
    centimeter as f32 * 28.345175603955806
}

fn to_pt_point(p_centimeter: Point) -> Point {
    p_centimeter * 28.345175603955806
}
