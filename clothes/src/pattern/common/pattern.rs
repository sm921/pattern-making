use pmdraw::{
    drawing::Drawing,
    shapes::{bezier::Bezier, line::Line},
};

use crate::pattern::measurements::Cm;

#[derive(Clone)]
enum Path {
    Line(Line),
    Curve(Bezier),
}

#[derive(Clone)]
struct Margin {
    path: Path,
    /// if true, not join the path with the next one
    skips_join: bool,
}

pub struct Pattern {
    paths: Vec<Path>,
    margin: Vec<Margin>,
}

impl Pattern {
    pub fn add_curve(&mut self, c: Bezier) -> () {
        self.paths.push(Path::Curve(c))
    }

    pub fn add_line(&mut self, l: Line) -> () {
        self.paths.push(Path::Line(l))
    }

    pub fn draw(&self, drawing: &mut Drawing) {
        for path in &self.paths {
            match &path {
                Path::Curve(c) => drawing.bezier(c),
                Path::Line(l) => drawing.line_borrow(l),
            };
        }
        for path in &self.margin {
            match &path.path {
                Path::Curve(c) => drawing.bezier(c),
                Path::Line(l) => drawing.line_borrow(l),
            };
        }
    }

    pub fn generate_margin(&mut self, margin: Cm) -> () {
        self.generate_margin_with_options(vec![margin; self.paths.len()], vec![])
    }

    pub fn generate_margin_diffrent_values(&mut self, margin: Vec<Cm>) -> () {
        self.generate_margin_with_options(margin, vec![])
    }

    pub fn generate_margin_except(&mut self, margin: Cm, except: Vec<usize>) -> () {
        self.generate_margin_with_options(vec![margin; self.paths.len()], except)
    }

    pub fn generate_margin_with_options(&mut self, margin: Vec<Cm>, except: Vec<usize>) -> () {
        assert_eq!(
            self.paths.len(),
            margin.len(),
            "margins must be specified for all the paths"
        );
        // add outer lines and curves
        for i in 0..self.paths.len() {
            let path = &self.paths[i];
            self.margin.push(Margin {
                path: match path {
                    Path::Curve(c) => Path::Curve(c.parallel(margin[i]).right),
                    Path::Line(l) => Path::Line(l.parallel(margin[i]).right),
                },
                skips_join: except.contains(&i),
            })
        }
        // join them
        let mut i = 0;
        while i < self.margin.len() {
            // consider two consecutive paths
            let path = &mut self.margin[i].clone();
            if path.skips_join {
                i += 1;
                continue;
            }
            let next_i = if i == &self.margin.len() - 1 {
                0
            } else {
                i + 1
            };
            let next_path = &mut self.margin[next_i].clone();
            // join consecutive paths and get a line between them if it exists
            let bridge_between_paths_to_concatenate_them: (Option<Line>, Option<Line>) =
                match &mut path.path {
                    Path::Line(l1) => match &mut next_path.path {
                        Path::Line(l2) => {
                            if l1.end != l2.origin {
                                l1.join(l2);
                            }
                            (None, None)
                        }
                        Path::Curve(c) => {
                            if l1.end != c.origin() {
                                (Some(l1.join_bezier(c)), None)
                            } else {
                                (None, None)
                            }
                        }
                    },
                    Path::Curve(c1) => match &mut next_path.path {
                        Path::Line(l) => {
                            if c1.end() != l.origin {
                                (Some(c1.join_line(l)), None)
                            } else {
                                (None, None)
                            }
                        }
                        Path::Curve(c2) => {
                            if c1.end() != c2.origin() {
                                let (l1, l2) = c1.join_by_extending(c2);
                                (Some(l1), Some(l2))
                            } else {
                                (None, None)
                            }
                        }
                    },
                };
            // update paths with joined ones
            self.margin[i] = path.clone();
            self.margin[next_i] = next_path.clone();
            match bridge_between_paths_to_concatenate_them {
                (Some(l), None) => {
                    self.margin.insert(
                        next_i,
                        Margin {
                            path: Path::Line(l),
                            skips_join: true,
                        },
                    );
                }
                (Some(l1), Some(l2)) => {
                    self.margin.insert(
                        next_i,
                        Margin {
                            path: Path::Line(l1),
                            skips_join: true,
                        },
                    );
                    self.margin.insert(
                        next_i + 1,
                        Margin {
                            path: Path::Line(l2),
                            skips_join: true,
                        },
                    );
                }
                _ => (),
            };
            i += 1;
        }
    }

    pub fn new() -> Pattern {
        Pattern {
            paths: Vec::new(),
            margin: Vec::new(),
        }
    }

    pub fn to(&mut self, dx: Cm, dy: Cm) -> () {
        for path in &mut self.paths {
            match path {
                Path::Curve(c) => c.to(dx, dy),
                Path::Line(l) => l.to(dx, dy),
            };
        }
        for path in &mut self.margin {
            match &mut path.path {
                Path::Curve(c) => c.to(dx, dy),
                Path::Line(l) => l.to(dx, dy),
            };
        }
    }
}
