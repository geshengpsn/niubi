use crate::{
    basics::{algorithm::de_casteljaul, ControlPoint},
    basis_function::BernsteinBasis,
};

use super::{NonRationalSurface, ParametricSurface};

struct BezierSurface<P>
where
    P: ControlPoint,
{
    u_basis_function: BernsteinBasis,
    v_basis_function: BernsteinBasis,
    control_points: Vec<Vec<P>>,
}

impl<P> BezierSurface<P>
where
    P: ControlPoint,
{
    pub fn new(control_points: Vec<Vec<P>>) -> Self {
        Self {
            u_basis_function: BernsteinBasis::new(control_points.len()),
            v_basis_function: BernsteinBasis::new(control_points[0].len()),
            control_points,
        }
    }
}

impl<P: ControlPoint> ParametricSurface<P> for BezierSurface<P> {
    type BasisFunction = BernsteinBasis;

    fn u_basis_function(&self) -> &Self::BasisFunction {
        &self.u_basis_function
    }

    fn v_basis_function(&self) -> &Self::BasisFunction {
        &self.v_basis_function
    }

    fn control_points(&self) -> &Vec<Vec<P>> {
        &self.control_points
    }
}

impl<P: ControlPoint> NonRationalSurface<P> for BezierSurface<P>
where
    P: ControlPoint,
{
    /// get geometry point from parameter u & v
    fn get_point(&self, u: f64, v: f64) -> P {
        let n = self.p();
        let m = self.q();
        if n <= m {
            let mut q = vec![P::zeros(); m + 1];
            for j in 0..=m {
                q[j] = de_casteljaul(n, u, &self.control_points[..][j]);
            }
            return de_casteljaul(m, v, &q);
        } else {
            let mut q = vec![P::zeros(); n + 1];
            for i in 0..=n {
                q[i] = de_casteljaul(m, v, &self.control_points[i]);
            }
            return de_casteljaul(n, u, &q);
        }
    }
}
