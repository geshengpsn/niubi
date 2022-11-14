use crate::basics::{ControlPoint, HomoControlPoint};
use crate::basis_function::Basis;

use self::algorithm::get_rational_ders;
use self::algorithm::{get_ders, get_point};

mod algorithm;
mod bezier;
mod bspline;
mod nurbs;

pub use bezier::AbstructBezier;
pub use bspline::AbstructBspline;
pub use nurbs::AbstructNURBS;

pub trait ParametricCurve<P>
{
    type BasisFunction: Basis;
    fn basis_function(&self) -> &Self::BasisFunction;
    fn control_points(&self) -> &Vec<P>;
    fn degree(&self) -> usize {
        self.basis_function().degree()
    }
}

/// 非有理曲线
pub trait NonRationalCurve<P>: ParametricCurve<P>
where
    P: ControlPoint,
{
    /// get geometry point from parameter
    fn get_point(&self, u: f64) -> P {
        get_point(self.basis_function(), self.control_points(), u)
    }

    /// get geometry point and ders from parameter
    /// der_upper_bond should <= p
    fn get_ders(&self, der_upper_bond: usize, u: f64) -> Vec<P> {
        get_ders(
            self.basis_function(),
            self.control_points(),
            der_upper_bond,
            u,
        )
    }
}

/// 有理曲线
pub trait RationalCurve<P>: ParametricCurve<HomoControlPoint<P>>
where
    P: ControlPoint,
{
    fn get_point(&self, u: f64) -> P {
        get_point(self.basis_function(), self.control_points(), u).to_control_point_and_weight().0
    }
    fn get_ders(&self, der_upper_bond: usize, u: f64) -> Vec<P> {
        get_rational_ders(self.basis_function(), self.control_points(), der_upper_bond, u)
    }
}
