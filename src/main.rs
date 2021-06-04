use pmdraw::drawing::Drawing;

fn main() {
    let drawing = Drawing::new();
    drawing.show(900, 900, -3.0..3.0, -1.0..1.0);
}
