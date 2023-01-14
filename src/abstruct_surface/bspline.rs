use crate::{basics::ControlPoint, basis_function::BsplineBasis};

use super::{NonRationalSurface, ParametricSurface};

struct BsplineSurface<P>
where
    P: ControlPoint,
{
    u_basis_function: BsplineBasis,
    v_basis_function: BsplineBasis,
    control_points: Vec<Vec<P>>,
}

impl<P> ParametricSurface<P> for BsplineSurface<P>
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

impl<P> NonRationalSurface<P> for BsplineSurface<P> where P: ControlPoint {}
