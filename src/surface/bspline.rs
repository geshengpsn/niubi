use crate::{basics::ControlPoint, basis_function::BsplineBasis};

use super::{NonRationalSurface, ParametricSurface};

pub struct BsplineSurfaceBase<P>
where
    P: ControlPoint,
{
    u_basis_function: BsplineBasis,
    v_basis_function: BsplineBasis,
    control_points: Vec<Vec<P>>,
}

impl<P> BsplineSurfaceBase<P>
where
    P: ControlPoint,
{
    /// TODO better error handle, unwarp() is bad
    /// 
    /// TODO refine all error handle
    pub fn new_uniform(control_points: Vec<Vec<P>>, u_degree: usize, v_degree: usize) -> Self {
        Self {
            u_basis_function: BsplineBasis::new_uniform(u_degree, control_points.len()).unwrap(),
            v_basis_function: BsplineBasis::new_uniform(v_degree, control_points[0].len()).unwrap(),
            control_points,
        }
    }

    /// if this work, user do no need to use trait to use basic algorithms
    pub fn get_point(&self) {
        <Self as NonRationalSurface<P>>::get_point(&self, 0., 0.);
    }
}

impl<P> ParametricSurface<P> for BsplineSurfaceBase<P>
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

    fn control_points(&self) -> &Vec<Vec<P>> {
        &self.control_points
    }
}

impl<P> NonRationalSurface<P> for BsplineSurfaceBase<P> where P: ControlPoint {}
