use crate::circle::Circle;

use super::{
    ralgo::{ralg5, ralgo_result_with_iterations},
    ralgo_params::RalgoParams,
    ralgo_result::RalgoResult,
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

pub fn dichotomy_step_ralgo_result_with_iterations(
    main_circle_radiuse: f32,
    circles: &Vec<Circle>,
    reset_step: bool,
    eps: f32,
    ralgo_params: &RalgoParams,
) -> RalgoResult {
    let mut x = circles_to_dvector(circles, main_circle_radiuse);
    let circles_radiuses =
        nalgebra::DVector::from_vec(Vec::from_iter(circles.iter().map(|c| c.radius)));

    let mut step_size = 40.96;
    let (mut ralgo_calls, mut total_iterations, mut total_calcfg_calls) = (0, 0, 0);

    while step_size >= 0.01 {
        let (iterations, calcfg_calls, y) = ralgo_result_with_iterations(
            x.clone(),
            ralgo_params.alpha,
            step_size.clone(),
            ralgo_params.q1,
            ralgo_params.epsx,
            ralgo_params.epsg,
            ralgo_params.max_iterations,
            &circles_radiuses,
        );

        ralgo_calls += 1;
        total_iterations += iterations;
        total_calcfg_calls += calcfg_calls;

        if (get_last(&x) - get_last(&y)) / get_last(&x) > eps {
            x = y;
            if reset_step {
                step_size = 40.96;
            }
        } else {
            step_size /= 2.0;
        }
    }

    let (main_circle_radius, circles) = dvector_to_answer(&x, &circles_radiuses);

    RalgoResult::new(
        ralgo_calls,
        total_iterations,
        total_calcfg_calls,
        main_circle_radius,
        circles,
    )
}
