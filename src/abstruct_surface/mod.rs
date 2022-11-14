use crate::{basics::ControlPoint};

mod bezier;

trait Surface<P> {
    type CP;
    fn get_point(&self, u: f64, v: f64) -> Self::CP;
}

trait ParametricSurface<P>
where
    P: ControlPoint,
{
    fn get_point(&self, u: f64, v: f64) -> P;
}

trait NonRationalSurface<P>: ParametricSurface<P>
where
    P: ControlPoint,
{
    /// get geometry point from parameter
    fn get_point(&self, u: f64, v: f64) -> P;
}
