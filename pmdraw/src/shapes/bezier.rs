use pmmath::{binomial::binomial, matrix::Mat};

use super::{
    line::Line,
    point::{sigma, Point},
};

/// A general bezier of n points
pub struct Bezier {
    /// fit points - used to culculate parallel lines
    pub fit_points: Vec<Point>,
    /// origin, end, and control points
    points: Vec<Point>,
    /// [Optional] - set range (from, to) to represent patial bezier curve
    pub range: RangePoint,
}

impl Bezier {
    /// derivative dB/dt = (dx/dt, dy/dt)
    pub fn derivative(&self, t: f64) -> Point {
        if t == 1.0 {
            (self.point_at(t) - self.point_at(t - 0.001)) / 0.001
        } else {
            (self.point_at(t + 0.001) - self.point_at(t)) / 0.001
        }
    }

    pub fn end(&self) -> Point {
        self.points[self.points.len() - 1]
    }

    pub fn join(&self, b: &Bezier) -> Line {
        self.range.to.line_to(b.range.from)
    }

    pub fn join_line(&mut self, l: &mut Line) -> Line {
        let mut bezier_edge = self
            .range
            .to
            .line_to(self.point_at(self.t_range().to + 0.01));
        l.join(&mut bezier_edge);
        bezier_edge
    }

    /// fit points
    pub fn new(fit_points: Vec<Point>) -> Bezier {
        let dt = 1.0 / (fit_points.len() as f64 - 1.0);
        let t_parameters = (0..fit_points.len())
            .map(|n| {
                if (n as f64) * dt > 1.0 {
                    1.0
                } else {
                    (n as f64) * dt
                }
            })
            .collect::<Vec<f64>>();
        Bezier::new_with_t(fit_points, t_parameters)
    }

    /// fit points and parameter values of each point

    pub fn new_with_t(fit_points: Vec<Point>, t: Vec<f64>) -> Bezier {
        let count_points = fit_points.len();

        // set origin and end
        let mut points = vec![Point::new(0.0, 0.0); count_points];
        let origin = fit_points[0];
        let end = fit_points[count_points - 1];
        points[0] = origin;
        points[count_points - 1] = end;

        // set ctrl points
        let ctrl_points = solve_ctrl_points(&fit_points, t);
        for i in 0..count_points - 2 {
            points[i + 1] = ctrl_points[i];
        }
        Bezier {
            fit_points,
            points,
            range: RangePoint {
                from: origin,
                to: end,
            },
        }
    }

    pub fn origin(&self) -> Point {
        self.points[0]
    }

    /// A general bezier of n points (P0, P1, ..., Pk) is definde as
    ///  B(t) = Sigma[ n-1Ck * (1-t)^(n-1-k) * t^k * Pk ]
    ///   where 0 <= t <= 1
    pub fn point_at(&self, t: f64) -> Point {
        // number of points
        let n = self.points.len();
        sigma(
            |k| {
                (1.0 - t).powf(n as f64 - 1.0 - k as f64)
                    * t.powf(k as f64)
                    * binomial(n - 1, k) as f64
                    * self.points[k]
            },
            0,
            n,
        )
    }

    pub fn set_range(&mut self, from: Point, to: Point) {
        self.range = RangePoint { from, to };
    }

    /// Solve parameter t of point p when p is somewhere on the curve by Newton's method
    pub fn solve_t_at(&self, p: Point) -> f64 {
        if p == self.origin() {
            return 0.0;
        }
        if p == self.end() {
            return 1.0;
        }
        let tolerance = 0.1;
        let (mut learning_rate_x, mut learning_rate_y) = (1.0, 1.0);
        // initial guess
        let mut t = 0.5;
        loop {
            // derivative at guess t
            let db_dt = self.derivative(t);
            // update t by Newton's method
            let p0 = self.point_at(t);
            let t1 = match t
                + learning_rate_y * (p.y - p0.y) / db_dt.y
                + learning_rate_x * (p.x - p0.x) / db_dt.x
            {
                // set uppder bound and lowerbound to prevent diverge
                t @ _ => {
                    if t > 1.0 {
                        1.0
                    } else if t < 0.0 {
                        0.0
                    } else {
                        t
                    }
                }
            };
            // validate new t
            let p1 = self.point_at(t1);
            let dif = p - p1;
            if dif.x.abs() < tolerance && dif.y.abs() < tolerance {
                return t1;
            }
            // if t is not precise enough, repeat the process against updated t
            t = t1;
            // modify learning rate to prevent surturation and divergence
            // learn more if difference is large
            learning_rate_x = dif.x.abs() / (dif.x.abs() + dif.y.abs());
            learning_rate_y = dif.y.abs() / (dif.x.abs() + dif.y.abs());
        }
    }

    /// Split the curve and get two curves
    pub fn split(&self, p: Point) -> Split {
        let mut b1 = self.clone();
        b1.set_range(self.origin(), p);
        let mut b2 = self.clone();
        b2.set_range(p, self.end());
        Split { fst: b1, snd: b2 }
    }

    /// Get range of t (0 to 1 by default)
    pub fn t_range(&self) -> RangeF64 {
        let range = self.range;
        let from = if range.from == self.origin() {
            0.0
        } else {
            self.solve_t_at(range.from)
        };
        let to = if range.to == self.end() {
            1.0
        } else {
            self.solve_t_at(range.to)
        };
        RangeF64 { from, to }
    }

    pub fn parallel(&self, distance: f64) -> Parallel {
        let make_parallel = |is_left: bool| {
            let mut fit_poitns = Vec::new();
            let mut range = self.range.clone();
            for i in 0..self.points.len() {
                let p = self.fit_points[i];
                let p_next = self.point_at(self.solve_t_at(self.fit_points[i]) + 0.01);
                let mut parallel_point = p_next.clone();
                parallel_point.rotate(if is_left { 90.0 } else { -90.0 }, p);
                parallel_point = p.to_point(parallel_point, distance);
                fit_poitns.push(parallel_point);
                if p == self.range.from {
                    range.from = parallel_point
                }
                if p == self.range.to {
                    range.to = parallel_point
                }
            }
            let mut parallel_bezier = Bezier::new(fit_poitns);
            parallel_bezier.range = range;
            parallel_bezier
        };
        Parallel {
            left: make_parallel(true),
            right: make_parallel(false),
        }
    }

    /// refit bezier curve with new fit_points
    ///
    /// range is reset uness explicity set
    pub fn refit<T>(
        &mut self,
        mut modify: T,
        range_from: Option<Point>,
        range_end: Option<Point>,
    ) -> ()
    where
        T: FnMut(&mut Vec<Point>) -> Vec<Point>,
    {
        let new_b = Bezier::new(modify(&mut self.fit_points));
        self.fit_points = new_b.fit_points;
        self.points = new_b.points;
        self.range.from = match range_from {
            Some(p) => p,
            None => self.range.from,
        };
        self.range.to = match range_end {
            Some(p) => p,
            None => self.range.to,
        };
    }

    /// Rotate around point
    pub fn rotate(&mut self, angle_degree: f64, around: Point) -> () {
        for i in 0..self.points.len() {
            self.points[i].rotate(angle_degree, around);
        }
        self.range.from.rotate(angle_degree, around);
        self.range.to.rotate(angle_degree, around);
    }

    pub fn to(&mut self, dx: f64, dy: f64) {
        for i in 0..self.fit_points.len() {
            self.fit_points[i] = self.fit_points[i].to(dx, dy);
        }
        for i in 0..self.points.len() {
            self.points[i] = self.points[i].to(dx, dy);
        }
        self.range.from = self.range.from.to(dx, dy);
        self.range.to = self.range.to.to(dx, dy);
    }
}

impl Clone for Bezier {
    fn clone(&self) -> Bezier {
        Bezier {
            fit_points: self.fit_points.clone(),
            points: self.points.clone(),
            range: self.range,
        }
    }
}

/// calculate control points from the points that the curve passes through  
///
///"algorithm to solve control points c1 to cn-2"
///   each ck (k=1, ..., n-2) is defined by 2 variables xk and yk;
///   thus,  Let c be vector of size 2(n-2) whose elemetns are {x1, y1, x2, y2,
///... , xn-2, yn-2}; and Let A be constraint matrix size 2(n-2) by 2(n-2); and
///Let b be vector of size 2(n-2) whose elemetns are expected values for each
///  constraints;
///
/// Because bezier curve passes through the points p1, ..., pn-2,
/// if t1,...,tn-2 are known,
/// then bezer satisfies 2(n-2) conditions as following,
///  p1 = Sigma { (1-t1)^(n-1-k) * t1^k n-1Ck fk }
///   ...
///  pn-2 = Sigma { (1-t)^(n-1-k) * t^k n-1Ck fk }
///  (k=0, ..., n-1, fk = p0 or c1,...,cn-2, or pn-1)
///
///  which is equal to the following conditions
///    p1 -p0 -pn-1 = Sigma { (1-t1)^(n-1-k) * t1^k n-1Ck pk }
///     ...
///    pn-2 -p0 -pn-1 = Sigma { (1-t)^(n-1-k) * t^k n-1Ck pk }
///    (k=1, ..., n-2)
///
///this filles all the rows of constraint matrix A and
///all the elements of the vector b,
///and control points are solved by a linear equasion A c = b
///thus,  c = A^-1 * b; */
fn solve_ctrl_points(points: &Vec<Point>, t: Vec<f64>) -> Vec<Point> {
    // number of points (p0, ..., pn-1)
    let n = points.len();

    // right hand side vector b where Ax = b
    let mut b = Mat::new(2 * (n - 2), 1);
    // constraints matrix A Ax = b)
    let mut a = Mat::new(2 * (n - 2), 2 * (n - 2));

    /*
    curve passes through points p1, ..., pn-2,
     implying that
       pi -(1-t)^(n-1) p0 - t^(n-1) pn-1
     is equal to
       Sigma { (1-ti)^(n-1-k) * ti^k n-1Ck pk }
     for
       k = 1, 2, ..., n-2
    */
    let p0 = points[0];
    let pn_1 = points[n - 1];
    for i in 1..n - 1 {
        let pi = points[i];
        let ti = t[i];
        for k in 1..n - 1 {
            // (1-ti)^(n-1-k) * ti^k n-1Ck pk
            let coefficient_k = (1.0 - ti).powf(n as f64 - 1.0 - k as f64)
                * ti.powf(k as f64)
                * binomial(n - 1, k) as f64;
            // pi -(1-t)^(n-1) p0 - t^(n-1) pn-1
            let right_hand_side =
                pi - ((1.0 - ti).powf(n as f64 - 1.0) * p0) - (ti.powf(n as f64 - 1.0) * pn_1);
            a[2 * i - 2][2 * k - 2] = coefficient_k;
            b[2 * i - 2][0] = right_hand_side.x;
            a[2 * i - 1][2 * k - 1] = coefficient_k;
            b[2 * i - 1][0] = right_hand_side.y;
        }
    }

    // solve
    let c = a.inverse().expect("Not invertible") * b;

    // convert to vector
    let mut ctrl_points = vec![Point::new(0.0, 0.0); n - 2];
    for i in 0..n - 2 {
        ctrl_points[i] = Point::new(c[2 * i][0], c[2 * i + 1][0]);
    }
    ctrl_points
}

pub struct Split {
    pub fst: Bezier,
    pub snd: Bezier,
}

#[derive(Copy, Clone)]
pub struct RangePoint {
    pub from: Point,
    pub to: Point,
}
pub struct RangeF64 {
    pub from: f64,
    pub to: f64,
}

pub struct Parallel {
    pub left: Bezier,
    pub right: Bezier,
}
