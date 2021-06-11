use pmdraw::drawing::Drawing;
use pmfile::pdf::pdf;

use crate::pattern::base::base::Base;

/// margin of the screen
const DRAWING_MARGIN: f64 = 9.0;

pub fn export_base(
    base: &Base,
    window_width: u32,
    window_height: u32,
    paper_width: Option<f64>,
    paper_height: Option<f64>,
    draws_margin: bool,
) -> () {
    let drawing_width = base.back.waist.len() + DRAWING_MARGIN;
    let drawing_height = base.front.shoulder.origin.y + DRAWING_MARGIN;

    let mut draw = Drawing::new(drawing_width, drawing_height);
    base.for_each_line(|l| draw.line(l));
    base.for_each_bezier(|b| draw.bezier(b));
    if draws_margin {
        let margin = base.margin();
        margin.base.for_each_line(|l| draw.line(l));
        margin.base.for_each_bezier(|b| draw.bezier(b));
    }

    // draw.show(window_width, window_height);
    pdf(&draw, paper_width, paper_height);
}
