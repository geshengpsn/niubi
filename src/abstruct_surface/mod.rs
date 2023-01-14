use crate::{
    basics::{
        algorithm::{get_surface_ders, get_surface_point},
        ControlPoint, HomoControlPoint,
    },
    basis_function::Basis,
};

use num::integer::binomial;

mod bezier;
mod bspline;
mod nurbs;

pub trait ParametricSurface<P>
where
    P: ControlPoint,
{
    type BasisFunction: Basis;
    // basis funtion u v
    fn u_basis_function(&self) -> &Self::BasisFunction;
    fn v_basis_function(&self) -> &Self::BasisFunction;
    fn control_points(&self) -> &Vec<Vec<P>>;
    fn p(&self) -> usize {
        self.u_basis_function().degree()
    }
    fn q(&self) -> usize {
        self.v_basis_function().degree()
    }
}

pub trait NonRationalSurface<P>: ParametricSurface<P>
where
    P: ControlPoint,
{
    /// get geometry point from parameter
    fn get_point(&self, u: f64, v: f64) -> P {
        get_surface_point(
            self.u_basis_function(),
            self.v_basis_function(),
            self.control_points(),
            u,
            v,
        )
    }

    fn get_ders(&self, der_upper_bond: usize, u: f64, v: f64) -> Vec<Vec<P>> {
        get_surface_ders(
            self.u_basis_function(),
            self.v_basis_function(),
            self.control_points(),
            der_upper_bond,
            u,
            v,
        )
    }
}

pub trait RationalSurface<P>: ParametricSurface<HomoControlPoint<P>>
where
    P: ControlPoint,
{
    /// get geometry point from parameter
    fn get_point(&self, u: f64, v: f64) -> P {
        get_surface_point(
            self.u_basis_function(),
            self.v_basis_function(),
            self.control_points(),
            u,
            v,
        )
        .to_control_point_and_weight()
        .0
    }

    fn get_ders(&self, der_upper_bond: usize, u: f64, v: f64) -> Vec<Vec<P>> {
        let homo_ders = get_surface_ders(
            self.u_basis_function(),
            self.v_basis_function(),
            self.control_points(),
            der_upper_bond,
            u,
            v,
        );

        let mut output = vec![vec![P::zeros(); der_upper_bond]; der_upper_bond];
        output[0][0] = homo_ders[0][0].to_control_point_and_weight().0;

        for k in 0..=der_upper_bond {
            for l in 0..der_upper_bond - k {
                let mut v = homo_ders[k][l].a();
                for j in 1..=l {
                    v -= output[k][l - j] * (binomial(l, j) as f64) * homo_ders[0][j].w()
                }
                for i in 1..=k {
                    v -= output[k - i][l] * (binomial(k, i) as f64) * homo_ders[i][0].w();
                    let mut v2 = P::zeros();
                    for j in 1..=l {
                        v2 += output[k - i][l - j] * (binomial(l, j) as f64) * homo_ders[i][j].w();
                    }
                    v -= v2 * (binomial(k, i) as f64)
                }
                output[k][l] = v / homo_ders[0][0].w();
            }
        }
        output
    }
}

#[test]
fn test_binomial() {
    for k in 0..=4 {
        let a = binomial(4, k);
        println!("{a}");
    }
}
