use crate::{
    basics::{algorithm::de_casteljaul, ControlPoint},
    basis_function::BernsteinBasis,
};

use super::{NonRationalCurve, ParametricCurve};

pub struct AbstructBezier<P>
where
    P: ControlPoint,
{
    basis_function: BernsteinBasis,
    control_points: Vec<P>,
}

impl<P> AbstructBezier<P>
where
    P: ControlPoint, 
{
    pub fn new(control_points: Vec<P>) -> Self {
        Self {
            basis_function: BernsteinBasis::new(control_points.len()),
            control_points,
        }
    }
}

impl<P> ParametricCurve<P> for AbstructBezier<P>
where
    P: ControlPoint,
{
    type BasisFunction = BernsteinBasis;
    fn basis_function(&self) -> &Self::BasisFunction {
        &self.basis_function
    }
    fn control_points(&self) -> &Vec<P> {
        &self.control_points
    }
}

impl<P> NonRationalCurve<P> for AbstructBezier<P>
where
    P: ControlPoint,
{
    /// use deCasteljaul algorithm
    fn get_point(&self, u: f64) -> P {
        de_casteljaul(self.degree(), u, &self.control_points)
    }
}
