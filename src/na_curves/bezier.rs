use nalgebra::SVector;

use crate::abstruct_curve::AbstructBezier;

pub type Bezier<const D: usize> = AbstructBezier<SVector<f64, D>>;

#[test]
fn test_bezier_curve_basis_function() {
    use crate::abstruct_curve::NonRationalCurve;
    use nalgebra::Vector2;
    let bezier = Bezier::new(vec![
        Vector2::new(200.0, 200.0),
        Vector2::new(300.0, 300.0),
        Vector2::new(500.0, 400.0),
        Vector2::new(600.0, 300.0),
    ]);
    assert_eq!(bezier.get_point(0.1), Vector2::new(232.8, 229.8));
    assert_eq!(
        bezier.get_point(0.3),
        Vector2::new(311.59999999999997, 284.6)
    );
    assert_eq!(bezier.get_point(0.5), Vector2::new(400.0, 325.0));
    assert_eq!(bezier.get_point(0.7), Vector2::new(488.4, 341.4));
    assert_eq!(bezier.get_point(0.9), Vector2::new(567.2, 324.2));
}

#[test]
fn test_bezier_curve_basis_get_point() {
    use crate::abstruct_curve::NonRationalCurve;
    use assert_approx_eq::assert_approx_eq;
    use nalgebra::Vector2;
    let bezier = Bezier::new(vec![
        Vector2::new(200.0, 200.0),
        Vector2::new(300.0, 300.0),
        Vector2::new(500.0, 400.0),
        Vector2::new(600.0, 300.0),
    ]);

    let ans = vec![
        Vector2::new(232.80000000000004, 229.80000000000007),
        Vector2::new(311.59999999999997, 284.6),
        Vector2::new(400.0, 325.0),
        Vector2::new(488.4, 341.4),
        Vector2::new(567.2, 324.2),
    ];

    for (index, &u) in [0.1, 0.3, 0.5, 0.7, 0.9].iter().enumerate() {
        let point = bezier.get_point(u);
        assert_approx_eq!(point.x, ans[index].x);
        assert_approx_eq!(point.y, ans[index].y);
    }
}

#[test]
fn test_bezier_curve_basis_get_ders() {
    use crate::abstruct_curve::{NonRationalCurve, ParametricCurve};
    use assert_approx_eq::assert_approx_eq;
    use nalgebra::Vector2;
    let bezier = Bezier::new(vec![
        Vector2::new(200.0, 200.0),
        Vector2::new(300.0, 300.0),
        Vector2::new(500.0, 400.0),
        Vector2::new(600.0, 300.0),
    ]);
    let ans = vec![
        Vector2::new(232.8, 229.8),
        Vector2::new(354.0, 294.0),
        Vector2::new(480.0, -120.0),
        Vector2::new(-1200.0, -1200.0),
    ];
    for (r, a) in bezier.get_ders(bezier.degree(), 0.1).iter().zip(ans.iter()) {
        assert_approx_eq!(r.x, a.x);
        assert_approx_eq!(r.y, a.y);
    }
}
