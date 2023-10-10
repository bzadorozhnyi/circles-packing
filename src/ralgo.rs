use crate::circle;
use crate::point;
use nalgebra::{self};

fn concat_gradients(
    gx: &nalgebra::DVector<f32>,
    gy: &nalgebra::DVector<f32>,
    gr: f32,
) -> nalgebra::DVector<f32> {
    let mut result_data = Vec::<f32>::with_capacity(2 * gx.len() + 1);
    for vector in [gx, gy] {
        for element in vector {
            result_data.push(*element);
        }
    }

    result_data.push(gr);

    return nalgebra::DVector::<f32>::from_vec(result_data);
}

fn calcfg(
    x: &nalgebra::DVector<f32>,
    radiuses: &nalgebra::DVector<f32>,
) -> (f32, nalgebra::DVector<f32>) {
    let number_of_circles = radiuses.len();

    let cx = x.rows(0, number_of_circles);
    let cy = x.rows(number_of_circles, number_of_circles);
    let main_circle_radius = x[x.len() - 1];

    let mut gx = nalgebra::DVector::<f32>::zeros(number_of_circles);
    let mut gy = nalgebra::DVector::<f32>::zeros(number_of_circles);
    let mut gr = 1_f32;

    let mut f = main_circle_radius;
    const P1: f32 = 2000.0;
    const P2: f32 = 1000.0;
    const EPS: f32 = 0.05;

    for i in 0..number_of_circles {
        let mut temp =
            cx[i].powi(2) + cy[i].powi(2) - (main_circle_radius - radiuses[i]).powi(2) + EPS;
        if temp > 0.0 {
            f += P1 * temp;
            gx[i] += P1 * cx[i];
            gy[i] += P1 * cy[i];
            gr -= P2;
        }

        for j in (i + 1)..number_of_circles {
            temp = -(cx[i] - cx[j]).powi(2) - (cy[i] - cy[j]).powi(2)
                + (radiuses[i] + radiuses[j]).powi(2)
                + EPS;
            if temp > 0.0 {
                f += P1 * temp;
                gx[i] -= P1 * (cx[i] - cx[j]);
                gy[i] -= P1 * (cy[i] - cy[j]);
                gx[j] += P1 * (cx[i] - cx[j]);
                gy[j] += P1 * (cy[i] - cy[j]);
            }
        }
    }

    let temp = -main_circle_radius + radiuses.min();
    if temp > 0.0 {
        f += P2 * temp;
        gr -= P2;
    }

    return (f, concat_gradients(&gx, &gy, gr));
}

fn ralg5(
    mut x: nalgebra::DVector<f32>,
    alpha: f32,
    mut h: f32,
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

fn circles_to_dvector(
    circles: &Vec<circle::Circle>,
    main_circle_radiuse: f32,
) -> nalgebra::DVector<f32> {
    let data: Vec<f32> = ([
        Vec::from_iter(
            circles
                .iter()
                .map(|c| c.center.as_ref().expect("Valid center").x),
        ),
        Vec::from_iter(
            circles
                .iter()
                .map(|c| c.center.as_ref().expect("Valid center").y),
        ),
        vec![main_circle_radiuse],
    ])
    .concat();
    return nalgebra::DVector::from_vec(data);
}

fn dvector_to_answer(
    x: &nalgebra::DVector<f32>,
    circles_radiuses: &nalgebra::DVector<f32>,
) -> (f32, Vec<circle::Circle>) {
    let main_circle_radiuse = x[x.len() - 1];
    let mut circles: Vec<circle::Circle> = Vec::from_iter(
        circles_radiuses
            .iter()
            .map(|radius| circle::Circle::new_with_radius(*radius)),
    );

    for i in 0..x.len() / 2 {
        circles[i].center = Some(point::Point {
            x: x[i],
            y: x[i + x.len() / 2],
        });
    }

    return (main_circle_radiuse, circles);
}

fn get_last(d: &nalgebra::DVector<f32>) -> f32 {
    return d[d.len() - 1];
}

pub fn dichotomy_step_ralgo(
    main_circle_radiuse: f32,
    circles: &Vec<circle::Circle>,
    reset_step: bool,
    eps: f32
) -> (f32, Vec<circle::Circle>) {
    let mut x = circles_to_dvector(circles, main_circle_radiuse);
    let circles_radiuses =
        nalgebra::DVector::from_vec(Vec::from_iter(circles.iter().map(|c| c.radius)));

    let mut step_size = 40.96;

    while step_size >= 0.01 {
        let y = ralg5(
            x.clone(),
            3.0,
            step_size.clone(),
            1e-6,
            1e-7,
            3000,
            &circles_radiuses,
        );

        // if get_last(&y) + eps < get_last(&x) {
        //     x = y;
        //     if reset_step {
        //         step_size = 40.96;
        //     }
        // }
        // else {
        //     step_size /= 2.0;
        // }

        if (get_last(&x) - get_last(&y)) / get_last(&x) > eps {
            x = y;
            if reset_step {
                step_size = 40.96;
            }
        }
        else {
            step_size /= 2.0;
        }
    }

    return dvector_to_answer(&x, &circles_radiuses);
}