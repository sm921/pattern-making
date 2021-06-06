use std::{fs::File, io::Write};

use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape},
};

const A3_WIDTH: f32 = 842.0;
const A3_HEIGHT: f32 = 1190.0;
const PRECISION: u32 = 100;

pub fn pdf(drawing: &Drawing) -> String {
    let mut out = String::from(
        "%PDF-1.7
%µí®û
",
    );
    // divide large drawings into multiple A3 pages
    // page_id starts from 3 because 1 and 2 are used for pages info
    let mut page_id = 3;
    // store page_id for info section
    let mut page_id_list = Vec::new();
    // store object_positions for xref
    let mut object_positions = Vec::new();
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
            // store id of not start but end of stream
            page_id_list.push(page_id + 2);
            let positions = write_stream(
                &mut out,
                drawing.shapes.clone(),
                page_id,
                offset_x,
                offset_y,
            );
            for position in positions {
                object_positions.push(position)
            }
            // stream contains 2 objects, thus next id is added by 3
            page_id += 3;
            if offset_x + A3_WIDTH > width {
                break;
            }
            j += 1.0;
        }
        if offset_y + A3_HEIGHT > height {
            break;
        }
        i += 1.0;
    }
    let object_end_position = out.as_bytes().len();
    write_info(&mut out, page_id_list);
    write_xref(&mut out, object_positions, object_end_position);
    out.push_str("%%EOF");
    let mut file = File::create("drawing.pdf").unwrap();
    file.write_all(out.as_bytes()).unwrap();
    out
}

fn draw_bezier(pdf: &mut String, b: Bezier, offset_x: f32, offset_y: f32) {
    let mut t = 0.0;
    let dt = 1.0 / PRECISION as f64;
    while t <= 1.0 {
        let p1 = b.point_at(t);
        let p2 = if t + dt < 1.0 {
            b.point_at(t + dt)
        } else {
            b.end()
        };
        draw_line(pdf, Line::new(p1, p2), offset_x, offset_y);
        t += dt;
    }
}

fn draw_circle(pdf: &mut String, c: Circle, offset_x: f32, offset_y: f32) {
    let origin = to_pt_point(c.origin).to(-offset_x as f64, -offset_y as f64, 0.0);
    let get_point =
        |theta: f64| origin + (to_pt(c.r) as f64 * Point::new(theta.cos(), theta.sin()));
    let mut theta: f64 = 0.0;
    let d_theta = 360.0 / (PRECISION as f64);
    let mut p0 = get_point(theta);
    loop {
        if theta + d_theta >= 360.0 {
            let p1 = get_point(360.0);
            return draw_line(pdf, Line::new(p0, p1), 0.0, 0.0);
        }
        theta = theta + d_theta;
        let p1 = get_point(theta);
        draw_line(pdf, Line::new(p0, p1), 0.0, 0.0);
        p0 = p1;
    }
}

fn draw_line(pdf: &mut String, mut l: Line, offset_x: f32, offset_y: f32) {
    l.origin = to_pt_point(l.origin).to(-offset_x as f64, -offset_y as f64, 0.0);
    l.end = to_pt_point(l.end).to(-offset_x as f64, -offset_y as f64, 0.0);
    pdf.push_str(&format!(
        "{} {} m {} {} l ",
        l.origin.x as f32, l.origin.y as f32, l.end.x as f32, l.end.y as f32
    ));
}

fn draw_point(pdf: &mut String, p: Point, offset_x: f32, offset_y: f32) {
    let p = to_pt_point(p).to(-offset_x as f64, -offset_y as f64, 0.0);
    draw_circle(pdf, Circle::new(p, 1.0), offset_x, offset_x);
}

fn to_pt(centimeter: f64) -> f32 {
    centimeter as f32 * 28.345175603955806
}

fn to_pt_point(p_centimeter: Point) -> Point {
    p_centimeter * 28.345175603955806
}

fn write_info(pdf: &mut String, page_id_list: Vec<usize>) -> () {
    pdf.push_str(&format!(
        "2 0 obj
<< /Type /Pages
   /Count {count_pages} 
   /Kids [ {page_id_list} ]
>>
endobj
1 0 obj
<< /Type /Catalog
   /Pages 2 0 R
>>
endobj
",
        count_pages = page_id_list.len(),
        page_id_list = page_id_list
            .iter()
            .map(|id| format!("{} 0 R ", id))
            .collect::<String>(),
    ));
}

/// return object positions
fn write_stream(
    pdf: &mut String,
    shapes: Vec<Shape>,
    page_index: usize,
    offset_x: f32,
    offset_y: f32,
) -> Vec<usize> {
    // begin
    pdf.push_str(&format!(
        "{id1} 0 obj
<< /Length {id2} 0 R >>
stream
",
        id1 = page_index,
        id2 = page_index + 1
    ));
    let start_position = pdf.as_bytes().len();
    pdf.push_str(
        "/DeviceRGB cs /DeviceRGB CS
0 0 0.972549 SC
",
    );
    // store object positions for xref
    let mut object_positions = Vec::new();
    for shape in shapes {
        object_positions.push(pdf.as_bytes().len());
        match shape {
            Shape::Bezier(b) => draw_bezier(pdf, b, offset_x, offset_y),
            Shape::Point(p) => draw_point(pdf, p, offset_x, offset_y),
            Shape::Line(l) => draw_line(pdf, l, offset_x, offset_y),
            Shape::Circle(c) => draw_circle(pdf, c, offset_x, offset_y),
        }
    }
    let content_length = pdf.as_bytes().len() - start_position;
    // end
    pdf.push_str(&format!(
        "S
endstream
endobj
{id2} 0 obj
{content_length}
endobj
{id3} 0 obj
<< /Type /Page
   /Parent 2 0 R
   /Resources << /Font << >> >>
   /MediaBox [ 0 0 {width} {height} ]
   /Contents {id1} 0 R
>>
endobj
",
        id2 = page_index + 1,
        content_length = content_length,
        id3 = page_index + 2,
        width = A3_WIDTH as u32,
        height = A3_HEIGHT as u32,
        id1 = page_index
    ));
    object_positions
}

/// Specify where the objects are located in bytes
/// * object_positions - object start positions in bytes
/// * object_end_position - last object's end position
fn write_xref(pdf: &mut String, object_positions: Vec<usize>, object_end_position: usize) {
    let count_objects = object_positions.len();
    // begin
    pdf.push_str(&format!(
        "xref
0 {}
0000000000 65535 f 
",
        count_objects
    ));

    for i in 0..count_objects {
        pdf.push_str(&format!("{:010} 00000 n ", object_positions[i]));
    }

    //end
    pdf.push_str(&format!(
        "trailer
<< /Size {count_objects}
   /Root 1 0 R
>>
startxref
{object_end_position}
",
        count_objects = count_objects,
        object_end_position = object_end_position
    ));
}
