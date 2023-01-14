use nalgebra::SVector;

use crate::surface::BezierSurfaceBase;

pub type BezierSurface<const D: usize> = BezierSurfaceBase<SVector<f64, D>>;