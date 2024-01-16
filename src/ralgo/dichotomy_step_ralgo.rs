use crate::circle::Circle;

use super::{
    ralgo::ralg5,
    ralgo_params::RalgoParams,
    utils::{circles_to_dvector, dvector_to_answer, get_last},
};

pub fn dichotomy_step_ralgo(
    main_circle_radiuse: f32,
    circles: &Vec<Circle>,
    reset_step: bool,
    eps: f32,
    ralgo_params: &RalgoParams,
) -> (f32, Vec<Circle>) {
    let mut x = circles_to_dvector(circles, main_circle_radiuse);
    let circles_radiuses =
        nalgebra::DVector::from_vec(Vec::from_iter(circles.iter().map(|c| c.radius)));

    let mut step_size = 40.96;

    while step_size >= 0.01 {
        let y = ralg5(
            x.clone(),
            ralgo_params.alpha,
            step_size.clone(),
            ralgo_params.q1,
            ralgo_params.epsx,
            ralgo_params.epsg,
            ralgo_params.max_iterations,
            &circles_radiuses,
        );

        if (get_last(&x) - get_last(&y)) / get_last(&x) > eps {
            x = y;
            if reset_step {
                step_size = 40.96;
            }
        } else {
            step_size /= 2.0;
        }
    }

    return dvector_to_answer(&x, &circles_radiuses);
}
