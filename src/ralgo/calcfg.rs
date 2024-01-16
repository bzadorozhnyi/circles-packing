use crate::ralgo::utils::concat_gradients;

pub fn calcfg(
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
