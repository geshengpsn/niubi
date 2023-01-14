use crate::{
    basics::{ControlPoint, HomoControlPoint},
    basis_function::BsplineBasis,
};

use super::{ParametricSurface, RationalSurface};

struct NurbsSurface<P>
where
    P: ControlPoint,
{
    u_basis_function: BsplineBasis,
    v_basis_function: BsplineBasis,
    control_points: Vec<Vec<HomoControlPoint<P>>>,
}

impl<P> ParametricSurface<HomoControlPoint<P>> for NurbsSurface<P>
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

impl<P> RationalSurface<P> for NurbsSurface<P> where P: ControlPoint {}
