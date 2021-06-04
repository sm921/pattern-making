use self::{bezier::Bezier, circle::Circle, line::Line, point::Point};

pub mod bezier;
pub mod circle;
pub mod line;
pub mod point;

#[derive(Clone)]
pub enum Shape {
    Bezier(Bezier),
    Point(Point),
    Line(Line),
    Circle(Circle),
}
