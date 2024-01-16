use nalgebra::{self};

use super::calcfg::calcfg;

pub fn ralg5(
    mut x: nalgebra::DVector<f32>,
    alpha: f32,
    mut h: f32,
    q1: f32,
    epsx: f32,
    epsg: f32,
    max_iterations: usize,
    radiuses: &nalgebra::DVector<f32>,
) -> nalgebra::DVector<f32> {
    let mut b_matrix = nalgebra::DMatrix::<f32>::identity(x.len(), x.len());

    let mut result_x = x.clone();
    let (mut result_f, mut g0) = calcfg(&result_x, radiuses);

    if g0.norm() < epsg {
        return result_x;
    }

    for _ in 0..max_iterations {
        let mut g1: nalgebra::DVector<f32> = b_matrix.tr_mul(&g0);

        let dx: nalgebra::DVector<f32> = (&b_matrix * &g1) / g1.norm();
        let dx_norm = dx.norm();

        let mut f;
        let (mut d, mut ls, mut ddx) = (1.0_f32, 0_u32, 0.0_f32);
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