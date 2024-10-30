use nalgebra::SVector;

use crate::surface::BsplineSurfaceBase;

pub type BsplineSurface<const D: usize> = BsplineSurfaceBase<SVector<f64, D>>;

/// TODO design test cases and TEST 
#[test]
fn test_bspline_surface_new() {
    use nalgebra::Vector4;
    BsplineSurface::new_uniform(
        vec![
            vec![Vector4::new(1., 1., 0., 1.), Vector4::new(-1., 1., 0., 1.)],
            vec![Vector4::new(1., 1., 1., 1.), Vector4::new(-1., 1., 1., 1.)],
            vec![Vector4::new(2., 2., 0., 2.), Vector4::new(-2., 2., 0., 2.)],
        ],
        2,
        1,
    );
}

#[test]
fn test_bspline_surface_get_point() {}


#[test]
fn test_bspline_surface_get_ders() {}