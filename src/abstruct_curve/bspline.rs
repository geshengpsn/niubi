use crate::basis_function::{BsplineBasis, Knots};

use super::{point::ControlPoint, NonRationalCurve, ParametricCurve};

pub struct AbstructBspline<P>
where
    P: ControlPoint,
{
    basis_function: BsplineBasis,
    control_points: Vec<P>,
}

impl<P> AbstructBspline<P>
where
    P: ControlPoint,
{
    pub fn new(control_points: Vec<P>, knots: Vec<f64>, degree: usize) -> Self {
        Self {
            basis_function: BsplineBasis::new(degree, Knots(knots)),
            control_points,
        }
    }

    /// panic if length of control_points < degree + 1
    pub fn new_uniform(control_points: Vec<P>, degree: usize) -> Self {
        Self {
            basis_function: BsplineBasis::new_uniform(degree, control_points.len()).unwrap(),
            control_points,
        }
    }
}

impl<P> ParametricCurve<BsplineBasis, P> for AbstructBspline<P>
where
    P: ControlPoint,
{
    fn basis_function(&self) -> &BsplineBasis {
        &self.basis_function
    }

    fn control_points(&self) -> &Vec<P> {
        &self.control_points
    }
}

impl<P> NonRationalCurve<BsplineBasis, P> for AbstructBspline<P> where P: ControlPoint {}
