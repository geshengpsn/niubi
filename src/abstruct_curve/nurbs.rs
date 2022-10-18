use crate::basis_function::{BsplineBasis, Knots};

use super::{
    point::{ControlPoint, HomoControlPoint},
    ParametricCurve, RationalCurve,
};

pub struct AbstructNURBS<P>
where
    P: HomoControlPoint,
{
    basis_function: BsplineBasis,
    control_points: Vec<P>,
}

impl<P> AbstructNURBS<P>
where
    P: HomoControlPoint,
{
    pub fn new(control_points_weights: Vec<(P::CP, f64)>, knots: Knots, degree: usize) -> Self {
        let control_points = control_points_weights
            .iter()
            .map(|(p, w)| P::from_control_point(*p, *w))
            .collect::<Vec<_>>();
        Self {
            basis_function: BsplineBasis::new(degree, knots),
            control_points,
        }
    }
}

impl<P> ParametricCurve<BsplineBasis, P> for AbstructNURBS<P>
where
    P: HomoControlPoint,
{
    fn basis_function(&self) -> &BsplineBasis {
        &self.basis_function
    }

    fn control_points(&self) -> &Vec<P> {
        &self.control_points
    }
}

impl<HP, P> RationalCurve<BsplineBasis, HP, P> for AbstructNURBS<HP>
where
    HP: HomoControlPoint<CP = P>,
    P: ControlPoint,
{
}
