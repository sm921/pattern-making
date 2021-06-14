use pmdraw::drawing::Drawing;
use pmfile::{pdf::pdf, pdf2::pdf2};

use crate::pattern::base::base::Base;

/// margin of the screen
const DRAWING_MARGIN: f64 = 9.0;

pub fn export_base(
    base: &Base,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
    draws_margin: bool,
) -> Drawing {
    let mut base = base.clone();
    // fit A3 size
    let fit_a3_width = 0.0; //29.7 + 3.0 + 1.0; // (A3 width) + (space between front and back) + (margin of back)
    base.back.to(fit_a3_width, 0.0);
    let drawing_width = fit_a3_width + base.back.waist.len() + DRAWING_MARGIN;
    let drawing_height = base.back.shoulder.origin.y + DRAWING_MARGIN;

    let mut draw = Drawing::new(drawing_width, drawing_height);
    base.for_each_line(|l| draw.line(l));
    base.for_each_bezier(|b| draw.bezier(b));
    if draws_margin {
        let margin = base.margin();
        margin.base.for_each_line(|l| draw.line(l));
        margin.base.for_each_bezier(|b| draw.bezier(b));
    }

    pdf("_base.pdf", &draw, paper_width, paper_height);
    pdf2("base.pdf", &draw, paper_width, paper_height);
    draw
}
