use std::{f64::consts::PI, ops::Range};

use pmrender::show_lines;

use crate::shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape};

pub struct CoordinatesRange {
    pub x: Range<f64>,
    pub y: Range<f64>,
    pub z: Range<f64>,
}

pub struct Drawing {
    /// canvas width in centimeters
    pub width: f64,
    /// canvas height in centimeters
    pub height: f64,
    pub shapes: Vec<Shape>,
    vertices: Vec<(f32, f32)>,
}

impl Drawing {
    pub fn new(width: f64, height: f64) -> Drawing {
        let shapes = Vec::new();
        let vertices = Vec::new();
        Drawing {
            width,
            height,
            shapes,
            vertices,
        }
    }

    pub fn draw_bezier(&mut self, b: Bezier, precision: u32) {
        self.shapes.push(Shape::Bezier(b.clone()));
        let mut t = 0.0;
        let dt = 1.0 / precision as f64;
        while t <= 1.0 {
            self.draw_line_no_store(Line::new(
                b.point_at(t),
                if t + dt < 1.0 {
                    b.point_at(t + dt)
                } else {
                    b.end()
                },
            ));
            t += dt;
        }
    }

    pub fn draw_circle(&mut self, origin: Point, r: f64) {
        self.draw_circle_with_precision(origin, r, 100)
    }
    pub fn draw_circle_with_precision(&mut self, origin: Point, r: f64, precision: u32) {
        let c = Circle::new(origin, r);
        self.shapes.push(Shape::Circle(c));
        let dt = 1.0 / (precision - 1) as f64;
        let mut p0 = origin.to(r, 0.0, 0.0);
        for i in 1..precision {
            let p1 = match i as f64 * dt {
                t @ _ => {
                    if t >= 1.0 {
                        origin.to(r, 0.0, 0.0)
                    } else {
                        let theta = 2.0 * PI * t;
                        origin.to(r * theta.cos(), r * theta.sin(), 0.0)
                    }
                }
            };
            self.draw_line_struct(Line::new(p0, p1));
            p0 = p1;
        }
    }

    pub fn draw_line(&mut self, origin_x: f64, origin_y: f64, end_x: f64, end_y: f64) {
        self.draw_line_with_store(
            Line::new(Point::new(origin_x, origin_y), Point::new(end_x, end_y)),
            true,
        );
    }
    pub fn draw_line_struct(&mut self, l: Line) {
        self.draw_line_with_store(l, true);
    }
    pub fn draw_line_no_store(&mut self, l: Line) {
        self.draw_line_with_store(l, false);
    }
    fn draw_line_with_store(&mut self, l: Line, stores_shape: bool) {
        if stores_shape {
            self.shapes.push(Shape::Line(l.clone()));
        }
        self.vertices.push((l.origin.x as f32, l.origin.y as f32));
        self.vertices.push((l.end.x as f32, l.end.y as f32));
    }

    pub fn draw_point(&mut self, p: Point) {
        self.shapes.push(Shape::Point(p));
        self.draw_circle_with_precision(p, 1.0, 20);
    }

    pub fn show(&self, window_width: u32, window_height: u32) {
        // normalize coordinates
        let scale = 2.0
            / (if self.width > self.height {
                self.width
            } else {
                self.height
            });
        let model = [
            [scale, 0.0, 0.0, 0.0],  // 1. column: normalize x coodinates
            [0.0, -scale, 0.0, 0.0], // 2. column: normalize and reverse y coodinates so that upperside is positive
            [0.0, 0.0, scale, 0.0],  // 3. column: normalize z coodinates
            [-0.9, 0.9, 0.0, 1.0],   // 4. column: move origin to the left bottom
        ];

        // show_lines(self.vertices.to_vec(), model, window_width, window_height);
    }
}
