///
// knot vector
use std::ops::Index;

use super::BasisFunctionError;

/// # trait of knot vector
/// knot vector provides find_span() and Index
pub struct Knots(pub(crate) Vec<f64>);

impl Knots {
    /// new a knot vector need user have knowledge on it
    /// do not public to user
    pub(crate) fn new(v: Vec<f64>) -> Self {
        Self(v)
    }

    pub fn m(&self) -> usize {
        self.0.len() - 1
    }

    /// find span of u in knot vector
    pub fn find_span(&self, degree: usize, u: f64) -> Result<usize, BasisFunctionError> {
        if u < self[0] {
            return Err(BasisFunctionError::ULessThanMin);
        }
        if &u > self.0.last().unwrap() {
            return Err(BasisFunctionError::UGreaterThanMax);
        }
        if u.is_infinite() {
            return Err(BasisFunctionError::UIsInfinite);
        }
        if u.is_nan() {
            return Err(BasisFunctionError::UIsNon);
        }
        let n = self.m() - degree - 1;
        // handle special case
        // 0, 0, 0, 1/2, 1, 1, 1
        //              ^
        //              u
        //
        if self.0.last().unwrap() == &u {
            return Ok(n);
        }

        let mut low = degree;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;

        while u < self[mid] || u >= self[mid + 1] {
            if u < self[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        Ok(mid)
    }
}

impl Index<usize> for Knots {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[test]
fn test_knots_m() {
    let v = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let knots = Knots::new(v);
    assert_eq!(knots.m(), 5);
}

#[test]
fn test_knots_index() {
    let v = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
    let knots = Knots::new(v);
    for i in 0..knots.0.len() {
        assert_eq!(knots[i], knots.0[i]);
    }
}

#[test]
fn test_knots_find_span_ok() {
    let v = vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 4.0, 5.0, 5.0, 5.0];
    let knots = Knots::new(v);
    let q_and_a = [
        (0.0, 2usize),
        (0.5, 2usize),
        (1.5, 3usize),
        (4.0, 7usize),
        (5.0, 7),
    ];
    for (q, a) in q_and_a {
        let span = knots.find_span(2, q);
        // println!("{span:?}");
        assert_eq!(span.unwrap(), a);
    }
}

#[test]
fn test_knots_find_span_err() {
    let v = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let knots = Knots::new(v);
    let qs = vec![
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        -0.1,
        0.0 - f64::EPSILON,
        1.0 + f64::EPSILON,
        2.0,
    ];
    for q in qs {
        let result = knots.find_span(2, q);
        println!("{q}");
        assert!(result.is_err());
    }
}
