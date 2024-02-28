use nalgebra::{self};

use super::calcfg::calcfg;

pub fn ralg5(
    mut x: nalgebra::DVector<f64>,
    alpha: f64,
    mut h: f64,
    q1: f64,
    epsx: f64,
    epsg: f64,
    max_iterations: usize,
    radiuses: &nalgebra::DVector<f64>,
) -> nalgebra::DVector<f64> {
    let mut b_matrix = nalgebra::DMatrix::<f64>::identity(x.len(), x.len());

    let mut result_x = x.clone();
    let (mut result_f, mut g0) = calcfg(&result_x, radiuses);

    if g0.norm() < epsg {
        return result_x;
    }

    for _ in 0..max_iterations {
        let mut g1: nalgebra::DVector<f64> = b_matrix.tr_mul(&g0);

        let dx: nalgebra::DVector<f64> = (&b_matrix * &g1) / g1.norm();
        let dx_norm = dx.norm();

        let mut f;
        let (mut d, mut ls, mut ddx) = (1.0_f64, 0_u32, 0.0_f64);
        while d > 0.0 {
            x -= h * &dx;
            ddx += h * dx_norm;

            (f, g1) = calcfg(&x, radiuses);
            if f < result_f {
                (result_f, result_x) = (f, x.clone());
            }

            if g1.norm() < epsg {
                return result_x;
            }

            ls += 1;
            if ls % 3 == 0 {
                h *= 1.1;
            }

            if ls > 500 {
                return result_x;
            }

            d = dx.dot(&g1);
        }

        if ls == 1 {
            h *= q1;
        }

        if ddx < epsx {
            return result_x;
        }

        let mut r = &b_matrix.transpose() * (&g1 - &g0);
        r /= r.norm();

        b_matrix += (1.0 / alpha - 1.0) * &b_matrix * &r * &r.transpose();
        g0 = g1;
    }

    return result_x;
}

pub fn ralgo_result_with_iterations(
    mut x: nalgebra::DVector<f64>,
    alpha: f64,
    mut h: f64,
    q1: f64,
    epsx: f64,
    epsg: f64,
    max_iterations: usize,
    radiuses: &nalgebra::DVector<f64>,
) -> (u32, u32, nalgebra::DVector<f64>) {
    let mut b_matrix = nalgebra::DMatrix::<f64>::identity(x.len(), x.len());

    let mut result_x = x.clone();
    let (mut result_f, mut g0) = calcfg(&result_x, radiuses);
    let mut calcfg_calls = 1_u32;

    if g0.norm() < epsg {
        return (0, calcfg_calls, result_x);
    }

    for iter in 0..max_iterations as u32 {
        let mut g1: nalgebra::DVector<f64> = b_matrix.tr_mul(&g0);

        let dx: nalgebra::DVector<f64> = (&b_matrix * &g1) / g1.norm();
        let dx_norm = dx.norm();

        let mut f;
        let (mut d, mut ls, mut ddx) = (1.0_f64, 0_u32, 0.0_f64);
        while d > 0.0 {
            x -= h * &dx;
            ddx += h * dx_norm;

            (f, g1) = calcfg(&x, radiuses);
            calcfg_calls += 1;
            if f < result_f {
                (result_f, result_x) = (f, x.clone());
            }

            if g1.norm() < epsg {
                return (iter, calcfg_calls, result_x);
            }

            ls += 1;
            if ls % 3 == 0 {
                h *= 1.1;
            }

            if ls > 500 {
                return (iter, calcfg_calls, result_x);
            }

            d = dx.dot(&g1);
        }

        if ls == 1 {
            h *= q1;
        }

        if ddx < epsx {
            return (iter, calcfg_calls, result_x);
        }

        let mut r = &b_matrix.transpose() * (&g1 - &g0);
        r /= r.norm();

        b_matrix += (1.0 / alpha - 1.0) * &b_matrix * &r * &r.transpose();
        g0 = g1;
    }

    return (max_iterations as u32, calcfg_calls, result_x);
}
