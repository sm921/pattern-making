use std::f64::consts::PI;

// #[cfg(not(target_arch = "wasm32"))]
// use pmrender::show_lines;

use crate::shapes::{bezier::Bezier, circle::Circle, line::Line, point::Point, Shape};

#[derive(Clone)]
pub struct Drawing {
    /// canvas width in centimeters
    pub width: f64,
    /// canvas height in centimeters
    pub height: f64,
    pub shapes: Vec<Shape>,
    vertices: Vec<(f32, f32)>,
}

impl Drawing {
    pub fn bezier(&mut self, b: &Bezier) {
        self.bezier_with_precision(b, 100)
    }
    pub fn bezier_with_precision(&mut self, b: &Bezier, precision: u32) {
        self.shapes.push(Shape::Bezier(b.clone()));
        let t_range = b.t_range();
        let mut t = t_range.0;
        let dt = t_range.1 / precision as f64;
        while t <= t_range.1 {
            self.line_no_store(Line::new(
                b.point_at(t),
                if t + dt < t_range.1 {
                    b.point_at(t + dt)
                } else {
                    b.point_at(t_range.1)
                },
            ));
            t += dt;
        }
    }

    pub fn circle(&mut self, origin: Point, r: f64) {
        self.circle_with_precision(origin, r, 100)
    }
    pub fn circle_with_precision(&mut self, origin: Point, r: f64, precision: u32) {
        let c = Circle::new(origin, r);
        self.shapes.push(Shape::Circle(c));
        let dt = 1.0 / (precision - 1) as f64;
        let mut p0 = origin.to(r, 0.0);
        for i in 1..precision {
            let p1 = match i as f64 * dt {
                t @ _ => {
                    if t >= 1.0 {
                        origin.to(r, 0.0)
                    } else {
                        let theta = 2.0 * PI * t;
                        origin.to(r * theta.cos(), r * theta.sin())
                    }
                }
            };
            self.line(Line::new(p0, p1));
            p0 = p1;
        }
    }

    pub fn line_from_point(&mut self, origin_x: f64, origin_y: f64, end_x: f64, end_y: f64) {
        self.line_with_store(
            Line::new(Point::new(origin_x, origin_y), Point::new(end_x, end_y)),
            true,
        );
    }
    pub fn line(&mut self, l: Line) {
        self.line_with_store(l, true);
    }
    fn line_no_store(&mut self, l: Line) {
        self.line_with_store(l, false);
    }
    fn line_with_store(&mut self, l: Line, stores_shape: bool) {
        if stores_shape {
            self.shapes.push(Shape::Line(l.clone()));
        }
        self.vertices.push((l.origin.x as f32, l.origin.y as f32));
        self.vertices.push((l.end.x as f32, l.end.y as f32));
    }

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

    pub fn point(&mut self, p: Point) {
        self.shapes.push(Shape::Point(p));
        self.circle_with_precision(p, 0.3, 20);
    }

    pub fn show(&self, _window_width: u32, _window_height: u32) {
        // normalize coordinates
        let scale = 2.0
            / (if self.width as f32 > self.height as f32 {
                self.width as f32
            } else {
                self.height as f32
            });
        let _model: [[f32; 4]; 4] = [
            [scale, 0.0, 0.0, 0.0],  // 1. column: normalize x coodinates
            [0.0, -scale, 0.0, 0.0], // 2. column: normalize and reverse y coodinates so that upperside is positive
            [0.0, 0.0, scale, 0.0],  // 3. column: normalize z coodinates
            [-0.9, 0.9, 0.0, 1.0],   // 4. column: move origin to the left bottom
        ];

        if cfg!(wasm32) {
            todo!()
        } else {
            // #[cfg(not(target_arch = "wasm32"))]
            // show_lines(
            //     self.vertices.to_vec(),
            //     _model,
            //     _window_width,
            //     _window_height,
            // )
        };
    }
}
