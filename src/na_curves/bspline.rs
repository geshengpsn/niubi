use nalgebra::SVector;

use crate::abstruct_curve::AbstructBspline;

pub type Bspline<const D: usize> = AbstructBspline<SVector<f64, D>>;

#[test]
fn test_bsplie_curve_get_point() {
    use nalgebra::Vector2;
    use crate::abstruct_curve::NonRationalCurve;
    let bspline = Bspline::new(
        vec![
            Vector2::new(200.0, 200.0),
            Vector2::new(300.0, 300.0),
            Vector2::new(500.0, 400.0),
            Vector2::new(600.0, 300.0),
        ],
        vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
        2,
    );
    assert_eq!(
        bspline.get_point(0.1),
        Vector2::new(240.00000000000006, 238.00000000000006)
    );
    assert_eq!(bspline.get_point(0.3), Vector2::new(320.0, 302.0));
    assert_eq!(bspline.get_point(0.5), Vector2::new(400.0, 350.0));
    assert_eq!(
        bspline.get_point(0.7),
        Vector2::new(479.99999999999994, 366.0)
    );
    assert_eq!(bspline.get_point(0.9), Vector2::new(560.0, 334.0));
}

#[test]
fn test_bsplie_curve_basis_get_ders() {
    use nalgebra::Vector2;
    use crate::abstruct_curve::NonRationalCurve;
    let bspline = Bspline::new_uniform(
        vec![
            Vector2::new(200.0, 200.0),
            Vector2::new(300.0, 300.0),
            Vector2::new(500.0, 400.0),
            Vector2::new(600.0, 300.0),
        ],
        2,
    );
    let ders = bspline.get_ders(2, 0.1);
    assert_eq!(
        ders[0],
        Vector2::new(240.00000000000006, 238.00000000000006)
    );
    assert_eq!(ders[1], Vector2::new(400.0000000000001, 360.0000000000001));
    assert_eq!(ders[2], Vector2::new(0.0, -400.0));
}
