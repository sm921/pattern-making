use std::ops::Range;

use pmrender::show_lines;

use crate::shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape};

pub struct CoordinatesRange {
    pub x: Range<f64>,
    pub y: Range<f64>,
    pub z: Range<f64>,
}

pub struct Drawing {
    shapes: Vec<Shape>,
    vertices: Vec<(f32, f32)>,
}

impl Drawing {
    pub fn new() -> Drawing {
        let shapes = Vec::new();
        let vertices = vec![
            (0.0, 0.0),
            (0.9, 0.0),
            (0.0, 0.5),
            (0.9, 0.5),
            (0.0, 0.9),
            (0.9, 0.9),
        ];
        Drawing { shapes, vertices }
    }

    pub fn draw(&mut self, shape: Shape, precision: u32) {
        self.shapes.push(shape.clone());
        match shape {
            Shape::Point(p) => self.draw_point(p),
            Shape::Bezier(b) => self.draw_bezier(b, precision),
            Shape::Line(l) => self.draw_line(l),
            Shape::Circle(c) => self.draw_circle(c, precision),
        }
    }

    pub fn show(
        &self,
        width: u32,
        height: u32,
        coordinate_range_x: Range<f32>,
        coordinate_range_y: Range<f32>,
    ) {
        let len_x = coordinate_range_x.end - coordinate_range_x.start;
        let len_y = coordinate_range_y.end - coordinate_range_y.start;
        // normalize coordinates
        let scale = 2.0 / (if len_x > len_y { len_x } else { len_y });
        let model = [
            [scale, 0.0, 0.0, 0.0], // 1. column
            [0.0, scale, 0.0, 0.0], // 2. column
            [0.0, 0.0, scale, 0.0], // 3. column
            [-0.9, 0.0, 0.9, 1.0],  // 4. column (left bottom is the origin)
        ];

        show_lines(self.vertices.to_vec(), model, width, height);
    }

    fn draw_bezier(&self, b: Bezier, precision: u32) {}

    fn draw_circle(&self, c: Circle, precision: u32) {}

    fn draw_line(&self, l: Line) {}

    fn draw_point(&self, p: Point) {}
}
