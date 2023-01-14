use nalgebra::SVector;

use crate::surface::NurbsSurfaceBase;

pub type NurbsSurface<const D: usize> = NurbsSurfaceBase<SVector<f64, D>>;