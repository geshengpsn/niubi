use crate::{
    basics::{ControlPoint, HomoControlPoint},
    basis_function::Basis,
};

/// de_casteljaul algorithm
///
pub(crate) fn de_casteljaul<P>(n: usize, u: f64, control_points: &Vec<P>) -> P
where
    P: ControlPoint,
{
    let mut q = control_points.clone();
    let u1 = 1.0 - u;
    for k in 1..=n {
        for i in 0..=(n - k) {
            q[i] = q[i] * u1 + q[i + 1] * u;
        }
    }
    q[0]
}

pub(crate) fn get_curve_point<B, P>(basis: &B, control_points: &Vec<P>, u: f64) -> P
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

pub(crate) fn get_surface_point<B, P>(
    u_basis: &B,
    v_basis: &B,
    control_points: &Vec<Vec<P>>,
    u: f64,
    v: f64,
) -> P
where
    B: Basis,
    P: ControlPoint,
{
    let knots = u_basis.knots();
    let p = u_basis.degree();
    let i = knots.find_span(p, u).unwrap();
    // N_i-p,p ... N_i,p
    let u_values = u_basis.get_values(u, i);

    let knots = v_basis.knots();
    let q = v_basis.degree();
    let j = knots.find_span(q, v).unwrap();
    // N_j-q,q ... N_j,q
    let v_values = v_basis.get_values(v, j);

    let mut temp = vec![P::zeros(); q + 1];
    for l in 0..=q {
        for k in 0..=p {
            temp[l] += control_points[i - p + k][j - q + l] * u_values[k];
        }
    }
    let mut res = P::zeros();
    for l in 0..=q {
        res += temp[l] * v_values[l];
    }
    res
}

pub(crate) fn get_curve_ders<B, P>(
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

pub(crate) fn get_surface_ders<B, P>(
    u_basis: &B,
    v_basis: &B,
    control_points: &Vec<Vec<P>>,
    der_upper_bond: usize,
    u: f64,
    v: f64,
) -> Vec<Vec<P>>
where
    B: Basis,
    P: ControlPoint,
{
    let p = u_basis.degree();
    let q = v_basis.degree();
    let mut output = vec![vec![P::zeros(); q + 1]; p + 1];
    let u_der_upper_bond = if der_upper_bond <= p {
        der_upper_bond
    } else {
        p
    };
    let v_der_upper_bond = if der_upper_bond <= q {
        der_upper_bond
    } else {
        q
    };
    let uspan = u_basis.get_span(u);
    let vspan = v_basis.get_span(v);
    let u_basis_ders = u_basis.get_ders(u, uspan);
    let v_basis_ders = v_basis.get_ders(v, vspan);
    for k in 0..=u_der_upper_bond {
        let mut temp = vec![P::zeros(); p + 1];
        for s in 0..=q {
            for r in 0..=p {
                temp[s] += control_points[uspan - p + r][vspan - q + s] * u_basis_ders[k][r];
            }
        }
        let dd = if der_upper_bond - k <= v_der_upper_bond {
            der_upper_bond - k
        } else {
            v_der_upper_bond
        };
        for l in 0..=dd {
            for s in 0..=q {
                output[k][l] += temp[s] * v_basis_ders[l][s];
            }
        }
    }
    output
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
    let homo_ders = get_curve_ders(basis, homo_control_points, du, u);
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
