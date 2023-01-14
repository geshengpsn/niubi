use nalgebra::SVector;

use crate::surface::BsplineSurfaceBase;

pub type BsplineSurface<const D: usize> = BsplineSurfaceBase<SVector<f64, D>>;
