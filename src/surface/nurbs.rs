use crate::{
    basics::{ControlPoint, HomoControlPoint},
    basis_function::{BsplineBasis, Knots},
};

use super::{ParametricSurface, RationalSurface};

pub struct NurbsSurfaceBase<P>
where
    P: ControlPoint,
{
    u_basis_function: BsplineBasis,
    v_basis_function: BsplineBasis,
    control_points: Vec<Vec<HomoControlPoint<P>>>,
}

impl<P> NurbsSurfaceBase<P>
where
    P: ControlPoint,
{
    pub fn new(
        control_points: Vec<Vec<HomoControlPoint<P>>>,
        u_degree: usize,
        u_knot: Vec<f64>,
        v_degree: usize,
        v_knot: Vec<f64>,
    ) -> Self {
        Self {
            u_basis_function: BsplineBasis::new(u_degree, Knots::new(u_knot)),
            v_basis_function: BsplineBasis::new(v_degree, Knots::new(v_knot)),
            control_points,
        }
    }

    pub fn new_uniform(
        control_points_weights: Vec<Vec<(P, f64)>>,
        u_degree: usize,
        v_degree: usize,
    ) -> Self {
        let control_points = control_points_weights
            .iter()
            .map(|v| {
                v.iter()
                    .map(|(p, w)| HomoControlPoint::<P>::from_control_point(*p, *w))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self {
            u_basis_function: BsplineBasis::new_uniform(u_degree, control_points.len()).unwrap(),
            v_basis_function: BsplineBasis::new_uniform(v_degree, control_points[0].len()).unwrap(),
            control_points,
        }
    }
}

impl<P> ParametricSurface<HomoControlPoint<P>> for NurbsSurfaceBase<P>
where
    P: ControlPoint,
{
    type BasisFunction = BsplineBasis;

    fn u_basis_function(&self) -> &Self::BasisFunction {
        &self.u_basis_function
    }

    fn v_basis_function(&self) -> &Self::BasisFunction {
        &self.v_basis_function
    }

    fn control_points(&self) -> &Vec<Vec<HomoControlPoint<P>>> {
        &self.control_points
    }
}

impl<P> RationalSurface<P> for NurbsSurfaceBase<P> where P: ControlPoint {}
