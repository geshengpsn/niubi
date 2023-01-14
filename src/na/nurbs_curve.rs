use nalgebra::SVector;

use crate::curve::NurbsCurveBase;

pub type NurbsCurve<const D: usize> = NurbsCurveBase<SVector<f64, D>>;

#[test]
fn test_nurbs_get_value() {
    use crate::curve::RationalCurve;
    use nalgebra::Vector2;
    let curve = NurbsCurve::<2>::new(
        vec![
            (Vector2::new(200.0, 200.0), 0.1),
            (Vector2::new(300.0, 300.0), 1.0),
            (Vector2::new(500.0, 400.0), 1.0),
            (Vector2::new(600.0, 300.0), 1.0),
        ],
        vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
        2,
    );
    assert_eq!(
        curve.get_point(0.1),
        Vector2::new(294.33962264150944, 289.62264150943395)
    );
}

#[test]
fn test_nurbs_get_ders() {
    use crate::curve::{ParametricCurve, RationalCurve};
    use nalgebra::Vector2;
    let curve = NurbsCurve::<2>::new(
        vec![
            (Vector2::new(1.0, 0.0), 1.0),
            (Vector2::new(1.0, 1.0), 1.0),
            (Vector2::new(0.0, 1.0), 2.0),
        ],
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
    );
    println!("{:?}", curve.get_ders(curve.degree(), 0.0));
    assert_eq!(
        curve.get_ders(curve.degree(), 0.0),
        vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 2.0),
            Vector2::new(-4.0, 0.0)
        ]
    );
}
