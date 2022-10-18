mod bezier;
mod bspline;
mod nurbs;

pub use bezier::Bezier;
pub use bspline::Bspline;
pub use nurbs::NURBS;

use nalgebra::SVector;

use crate::abstruct_curve::{ControlPoint, HomoControlPoint};

pub type Point<const D: usize> = SVector<f64, D>;

impl<const D: usize> ControlPoint for Point<D> {
    fn zeros() -> Self {
        SVector::<f64, D>::zeros()
    }
}

impl HomoControlPoint for Point<3> {
    type CP = SVector<f64, 2>;
    fn split(&self)->(Self::CP, f64) {
        (SVector::<f64, 2>::new(self.x, self.y), self.z)
    }

    fn from_control_point(control_point: Self::CP, wight: f64) -> Self {
        SVector::<f64, 3>::new(control_point.x * wight, control_point.y * wight, wight)
    }
}

impl HomoControlPoint for Point<4> {
    type CP = SVector<f64, 3>;

    fn split(&self)->(Self::CP, f64) {
        (SVector::<f64, 3>::new(self.x, self.y, self.z), self.w)
    }

    fn from_control_point(control_point: Self::CP, wight: f64) -> Self {
        SVector::<f64, 4>::new(
            control_point.x * wight,
            control_point.y * wight,
            control_point.z * wight,
            wight,
        )
    }

    
}
