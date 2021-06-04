use pmmath::{binomial::binomial, matrix::Mat};

use super::point::{sigma, Point};

pub struct Bezier {
    points: Vec<Point>,
}

impl Bezier {
    pub fn end(&self) -> Point {
        self.points[self.points.len()]
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
        let mut points = Vec::with_capacity(count_points);
        points[0] = fit_points[0];
        points[count_points - 1] = fit_points[count_points - 1];

        // set ctrl points
        let ctrl_points = solve_ctrl_points(fit_points, t);
        for i in 1..count_points - 1 {
            points[i] = ctrl_points[i];
        }
        Bezier { points }
    }

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

    pub fn origin(&self) -> Point {
        self.points[0]
    }
}

impl Clone for Bezier {
    fn clone(&self) -> Bezier {
        let mut copy_points = Vec::with_capacity(self.points.len());
        for i in 0..self.points.len() {
            copy_points[i] = self.points[i];
        }
        Bezier {
            points: copy_points,
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
fn solve_ctrl_points(points: Vec<Point>, t: Vec<f64>) -> Vec<Point> {
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
    let mut ctrl_points = Vec::with_capacity(c.count_rows);
    for i in 0..c.count_rows {
        ctrl_points[i] = Point::new(c[2 * i - 2][0], c[2 * i - 1][0]);
    }
    ctrl_points
}
