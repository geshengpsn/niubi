use nalgebra::SVector;

use crate::surface::BezierSurfaceBase;

pub type BezierSurface<const D: usize> = BezierSurfaceBase<SVector<f64, D>>;

#[test]
fn test_bezier_surface_new() {
    use crate::basis_function::Basis;
    use crate::surface::ParametricSurface;
    use nalgebra::Vector4;
    let surface = BezierSurface::new(vec![
        vec![Vector4::new(1., 1., 0., 1.), Vector4::new(-1., 1., 0., 1.)],
        vec![Vector4::new(1., 1., 1., 1.), Vector4::new(-1., 1., 1., 1.)],
        vec![Vector4::new(2., 2., 0., 2.), Vector4::new(-2., 2., 0., 2.)],
    ]);
    assert_eq!(
        surface.u_basis_function().knots().0,
        vec![0., 0., 0., 1., 1., 1.,]
    );
    assert_eq!(surface.v_basis_function().knots().0, vec![0., 0., 1., 1.]);
    assert_eq!(surface.u_basis_function().degree(), 2);
    assert_eq!(surface.v_basis_function().degree(), 1);
}

#[test]
fn test_bezier_surface_get_point() {
    use crate::surface::NonRationalSurface;
    use nalgebra::Vector4;
    let surface = BezierSurface::new(vec![
        vec![Vector4::new(1., 1., 0., 1.), Vector4::new(-1., 1., 0., 1.)],
        vec![Vector4::new(1., 1., 1., 1.), Vector4::new(-1., 1., 1., 1.)],
        vec![Vector4::new(2., 0., 2., 2.), Vector4::new(-2., 0., 2., 2.)],
    ]);
    assert_eq!(
        surface.get_point(0.5, 0.5),
        Vector4::new(0., 0.75, 1., 1.25)
    );
}
