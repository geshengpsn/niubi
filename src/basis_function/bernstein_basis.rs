use super::{knots::Knots, Basis};

pub struct BernsteinBasis {
    degree: usize,
    knots: Knots,
}

impl BernsteinBasis {
    /// create a bernstein basis function by input the number of control points
    /// 
    /// degree = n - 1
    /// 
    /// n will be to set 1 if give it 0
    pub fn new(n: usize) -> Self {
        let n = if n == 0 { 1 } else { n };
        let v = [vec![0.0; n], vec![1.0; n]].concat();
        let knots = Knots::new(v);
        Self { degree: n - 1, knots }
    }
}

impl Basis for BernsteinBasis {
    fn degree(&self) -> usize {
        self.degree
    }

    fn knots(&self) -> &Knots {
        &self.knots
    }
}

#[test]
fn test_bernstein_basis_new() {
    let b = BernsteinBasis::new(0);
    assert_eq!(b.knots.0, [vec![0.0; 1], vec![1.0; 1]].concat());
    for n in 1..=10 {
        let b = BernsteinBasis::new(n);
        assert_eq!(b.knots.0, [vec![0.0; n], vec![1.0; n]].concat());
    }
}

#[test]
fn test_bernstein_basis_degree() {
    for n in 1..=10 {
        let b = BernsteinBasis::new(n);
        assert_eq!(n - 1, b.degree);
    }
}

