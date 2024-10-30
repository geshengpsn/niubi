use nalgebra::SVector;

use crate::surface::NurbsSurfaceBase;

pub type NurbsSurface<const D: usize> = NurbsSurfaceBase<SVector<f64, D>>;

#[test]
fn test_nurbs_surface_new() {
    use nalgebra::Vector3;
    NurbsSurface::new_uniform(
        vec![
            vec![
                (Vector3::new(1., 1., 0.), 1.),
                (Vector3::new(1., 1., 0.), 1.),
            ],
            vec![
                (Vector3::new(1., 1., 0.), 1.),
                (Vector3::new(1., 1., 0.), 1.),
            ],
            vec![
                (Vector3::new(1., 1., 0.), 1.),
                (Vector3::new(1., 1., 0.), 1.),
            ],
        ],
        2,
        1,
    );
}

#[test]
fn test_nurbs_surface_get_point() {}

#[test]
fn test_nurbs_surface_get_ders() {}
