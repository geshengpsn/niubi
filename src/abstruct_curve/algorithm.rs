use crate::{basis_function::Basis, basics::{ControlPoint, HomoControlPoint}};

pub(crate) fn get_point<B, P>(basis: &B, control_points: &Vec<P>, u: f64) -> P
where
    B: Basis,
    P: ControlPoint,
{
    let knots = basis.knots();
    let p = basis.degree();
    let i = knots.find_span(p, u).unwrap();
    let ns = basis.get_values(u, i);
    let mut res = P::zeros();
    for j in 0..=p {
        res += control_points[i - p + j] * ns[j];
    }
    res
}

pub(crate) fn get_ders<B, P>(
    basis: &B,
    control_points: &Vec<P>,
    der_upper_bond: usize,
    u: f64,
) -> Vec<P>
where
    B: Basis,
    P: ControlPoint,
{
    let knots = basis.knots();
    let p = basis.degree();
    let du = if der_upper_bond <= p {
        der_upper_bond
    } else {
        p
    };
    let mut ck = vec![P::zeros(); du + 1];
    let span = knots.find_span(p, u).unwrap();
    let nders = basis.get_ders(u, span);
    for k in 0..=du {
        for j in 0..=p {
            ck[k] += control_points[span - p + j] * nders[k][j];
        }
    }
    ck
}

pub(crate) fn get_rational_ders<B, P>(
    basis: &B,
    homo_control_points: &Vec<HomoControlPoint<P>>,
    der_upper_bond: usize,
    u: f64,
) -> Vec<P>
where
    B: Basis,
    P: ControlPoint,
{
    let p = basis.degree();
    let du = if der_upper_bond <= p {
        der_upper_bond
    } else {
        p
    };
    // get Aders and wders
    let homo_ders = get_ders(basis, homo_control_points, du, u);
    // Aders, wders
    let (a_ders, w_ders) =
        homo_ders
            .iter()
            .fold((Vec::new(), Vec::new()), |(mut a_ders, mut w_ders), hp| {
                let (a, w) = hp.split();
                a_ders.push(a);
                w_ders.push(w);
                (a_ders, w_ders)
            });
    let mut ck = homo_ders
        .iter()
        .map(|hp| hp.to_control_point_and_weight().0)
        .collect::<Vec<_>>();
    for k in 0..=du {
        let mut v = a_ders[k];
        for i in 1..=k {
            v = v - ck[k - i] * (num::integer::binomial::<usize>(k, i) as f64) * w_ders[i];
        }
        ck[k] = v / w_ders[0];
    }

    ck
}
