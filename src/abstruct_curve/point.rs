use std::ops::*;

pub trait ControlPoint:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + Copy
    + Clone
{
    /// origin
    fn zeros() -> Self;
}

pub trait HomoControlPoint: ControlPoint {
    type CP: ControlPoint;
    /// split homo control point into 2 parts without modify value
    fn split(&self)->(Self::CP, f64);
    fn to_control_point_and_weight(&self) -> (Self::CP, f64) {
        let (fat_cp, w) = self.split();
        (fat_cp / w, w)
    }
    fn from_control_point(control_point: Self::CP, wight: f64) -> Self;
}

// impl<const D: usize, const E: usize> HomoControlPoint<E> for SVector<f64, D> {
//     fn to_control_point(&self) -> SVector<f64, E> {
//         let w = self[self.len() - 1];
//         let mut res = SVector::<f64, E>::zeros();
//         for i in 0..E {
//             res[i] = self[i] / w;
//         }
//         res
//     }

//     fn from_control_point(control_point: &SVector<f64, E>, wight: f64) -> Self {
//         let mut res = SVector::<f64, D>::zeros();
//         for i in 0..E {
//             res[i] = control_point[i] * wight;
//         }
//         res[E] = wight;
//         res
//     }
// }
