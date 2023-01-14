use crate::{
    basics::{ControlPoint, HomoControlPoint},
    basis_function::{BsplineBasis, Knots},
};

use super::{ParametricCurve, RationalCurve};

pub struct NurbsCurveBase<P>
where
    P: ControlPoint,
{
    basis_function: BsplineBasis,
    control_points: Vec<HomoControlPoint<P>>,
}

impl<P> NurbsCurveBase<P>
where
    P: ControlPoint,
{
    pub fn new(control_points_weights: Vec<(P, f64)>, knots: Vec<f64>, degree: usize) -> Self {
        let control_points = control_points_weights
            .iter()
            .map(|(p, w)| HomoControlPoint::<P>::from_control_point(*p, *w))
            .collect::<Vec<_>>();
        Self {
            basis_function: BsplineBasis::new(degree, Knots(knots)),
            control_points,
        }
    }
}

impl<P> ParametricCurve<HomoControlPoint<P>> for NurbsCurveBase<P>
where
    P: ControlPoint,
{
    type BasisFunction = BsplineBasis;
    fn basis_function(&self) -> &Self::BasisFunction {
        &self.basis_function
    }

    fn control_points(&self) -> &Vec<HomoControlPoint<P>> {
        &self.control_points
    }
}

impl<P> RationalCurve<P> for NurbsCurveBase<P> where P: ControlPoint {}
