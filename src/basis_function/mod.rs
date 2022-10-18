mod bernstein_basis;
mod bspline_basis;
mod knots;

use std::collections::HashMap;

pub use self::knots::Knots;
pub use bernstein_basis::BernsteinBasis;
pub use bspline_basis::BsplineBasis;

#[derive(Debug)]
pub enum BasisFunctionError {
    ULessThanMin,
    UGreaterThanMax,
    UIsInfinite,
    UIsNon,
    TooLargeDegree,
}

pub trait Basis {
    fn degree(&self) -> usize;

    fn knots(&self) -> &Knots;

    fn get_span(&self, u: f64) -> usize {
        self.knots().find_span(self.degree(), u).unwrap()
    }

    fn get_values(&self, u: f64, i: usize) -> Vec<f64>
    where
        Self:,
    {
        let p = self.degree();
        let knot_vector = self.knots();
        let mut basis = vec![0.0; p + 1];
        basis[0] = 1.0;
        for _p in 1..=p {
            let mut saved = 0.0;
            for r in 0.._p {
                let temp = basis[r] / (knot_vector[i + r + 1] - knot_vector[i + 1 - _p + r]);
                basis[r] = saved + (knot_vector[i + r + 1] - u) * temp;
                saved = (u - knot_vector[i + 1 - _p + r]) * temp;
            }
            basis[_p] = saved;
        }
        basis
    }

    /// - u : parameter u
    /// - i : span of u
    fn get_ders(&self, u: f64, i: usize) -> Vec<Vec<f64>> {
        let p = self.degree();
        let knot_vector = self.knots();
        // 构造微分结果矩阵 der
        let mut der = vec![vec![0.0; p + 1]; p + 1];
        // 构造 ndu
        let mut ndu = HashMap::with_capacity((p + 1) * (p + 1));
        ndu.insert([i, 0], 1.0);

        // 构造 a
        let mut a = HashMap::with_capacity((p + 1) * (p + 1) * (p + 1));
        for _p in 0..=p {
            a.insert([i - _p, 0, 0], 1.0);
        }
        for _i in i - p..=i {
            for k in 1..=p {
                for j in 0..k {
                    if j == 0 {
                        a.insert(
                            [_i, k, j],
                            a.get(&[_i, k - 1, j]).unwrap()
                                / (knot_vector[_i + p - k + 1] - knot_vector[_i]),
                        );
                    } else {
                        a.insert(
                            [_i, k, j],
                            (a.get(&[_i, k - 1, j]).unwrap() - a.get(&[_i, k - 1, j - 1]).unwrap())
                                / (knot_vector[_i + j + p - k + 1] - knot_vector[_i + j]),
                        );
                    }
                    a.insert(
                        [_i, k, k],
                        -a.get(&[_i, k - 1, k - 1]).unwrap()
                            / (knot_vector[_i + p + 1] - knot_vector[_i + k]),
                    );
                }
            }
        }

        for j in 1..=p {
            let mut saved = 0.0;
            for r in 0..j {
                let temp = ndu.get(&[i + r - j + 1, j - 1]).unwrap()
                    / (knot_vector[i + r + 1] - knot_vector[i + 1 - j + r]);
                ndu.insert([i + r - j, j], saved + (knot_vector[i + r + 1] - u) * temp);
                saved = (u - knot_vector[i + 1 - j + r]) * temp;
            }
            ndu.insert([i, j], saved);
        }

        let ndu = ndu;
        let a = a;

        // k=0
        for j in 0..=p {
            der[0][j] = *ndu.get(&[i - p + j, p]).unwrap();
        }

        // 计算微分
        // function index N_{i-r,p}
        for r in 0..=p {
            let _i = i - r;
            // N^{(k)}_{_i,p}
            for k in 1..=p {
                let mut pre = 1.0;
                for z in p - k + 1..=p {
                    pre *= z as f64;
                }
                let mut sum = 0.0;
                for j in 0..=k {
                    sum += match ndu.get(&[_i + j, p - k]) {
                        Some(v) => a.get(&[_i, k, j]).unwrap() * *v,
                        None => 0.0,
                    };
                }
                der[k][p - r] = pre * sum;
            }
        }
        der
    }
}

#[cfg(test)]
mod test {
    use super::{Basis, BernsteinBasis, BsplineBasis};
    // BernsteinBasis test
    #[test]
    fn test_bernstein_basis_get_value() {
        let b = BernsteinBasis::new(2);
        assert_eq!(b.get_values(0.5, b.get_span(0.5)), vec![0.5, 0.5]);
    }

    #[test]
    fn test_bernstein_basis_get_ders() {
        let u = 0.5;
        let b = BernsteinBasis::new(2);
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(ders, vec![vec![0.5, 0.5], vec![-1.0, 1.0]]);

        let b = BernsteinBasis::new(3);
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(
            ders,
            vec![
                vec![0.25, 0.5, 0.25],
                vec![-1.0, 0.0, 1.0],
                vec![2.0, -4.0, 2.0]
            ]
        );

        let b = BernsteinBasis::new(3);
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(
            ders,
            vec![
                vec![0.25, 0.5, 0.25],
                vec![-1.0, 0.0, 1.0],
                vec![2.0, -4.0, 2.0]
            ]
        );

        let b = BernsteinBasis::new(4);
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(
            ders,
            vec![
                vec![0.125, 0.375, 0.375, 0.125],
                vec![-0.75, -0.75, 0.75, 0.75],
                vec![3.0, -3.0, -3.0, 3.0],
                vec![-6.0, 18.0, -18.0, 6.0],
            ]
        );
    }

    // Bspline test
    #[test]
    fn test_bspline_basis_get_value() {
        let u = 0.5;
        let b = BsplineBasis::new_uniform(2, 4).unwrap();
        let span = b.get_span(u);
        let ders = b.get_values(u, span);
        assert_eq!(ders, vec![0.5, 0.5, 0.0]);

        let b = BsplineBasis::new_uniform(3, 4).unwrap();
        let span = b.get_span(u);
        let ders = b.get_values(u, span);
        assert_eq!(ders, vec![0.125, 0.375, 0.375, 0.125]);
    }

    #[test]
    fn test_bspline_basis_get_ders() {
        let u = 0.5;
        let b = BsplineBasis::new_uniform(2, 4).unwrap();
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(ders, vec![
            vec![0.5, 0.5, 0.0],
            vec![-2.0, 2.0, 0.0],
            vec![4.0, -12.0, 8.0],
        ]);

        let b = BsplineBasis::new_uniform(3, 5).unwrap();
        let span = b.get_span(u);
        let ders = b.get_ders(u, span);
        assert_eq!(ders, vec![
            vec![0.25, 0.5, 0.25, 0.0],
            vec![-1.5, 0.0, 1.5, 0.0],
            vec![6.0, -12.0, 6.0, 0.0],
            vec![-12.0, 48.0, -84.0, 48.0],
        ]);
    }
}
