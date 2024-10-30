use super::{Basis, BasisFunctionError, Knots};

pub struct BsplineBasis {
    degree: usize,
    knots: Knots,
}

impl BsplineBasis {
    ///create any bspline basis function
    /// TODO input only knots 
    pub fn new(degree: usize, knots: Knots) -> Self {
        Self { degree, knots }
    }
    ///create a bspline basis function by input the number of control points and degree of basis
    ///
    /// constraint: degree + 1 <= n
    pub fn new_uniform(degree: usize, n: usize) -> Result<Self, BasisFunctionError> {
        if degree + 1 > n {
            return Err(BasisFunctionError::TooLargeDegree);
        }
        // m = n + p + 1
        // 0.0,...,0.0, ... ,1.0,...,1.0
        //    p+1              p+1

        let mut v = Vec::new();
        let len = n - degree;
        for i in 0..n - degree + 1 {
            v.push(i as f64 / len as f64);
        }

        let knots = Knots::new([vec![0.0; degree], v, vec![1.0; degree]].concat());
        Ok(Self { degree, knots })
    }
}

impl Basis for BsplineBasis {
    fn degree(&self) -> usize {
        self.degree
    }

    fn knots(&self) -> &Knots {
        &self.knots
    }
}

#[test]
fn test_new_average_err() {
    let b = BsplineBasis::new_uniform(3, 2);
    assert!(b.is_err());
    let b = BsplineBasis::new_uniform(2, 2);
    assert!(b.is_err());
    let b = BsplineBasis::new_uniform(1, 2);
    assert!(b.is_ok());
}

#[test]
fn test_new_average_ok() {
    let b = BsplineBasis::new_uniform(2, 3).unwrap();
    assert_eq!(b.knots.0, vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
    let b = BsplineBasis::new_uniform(2, 4).unwrap();
    assert_eq!(b.knots.0, vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0]);
    let b = BsplineBasis::new_uniform(2, 5).unwrap();
    assert_eq!(
        b.knots.0,
        vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0]
    );
}
