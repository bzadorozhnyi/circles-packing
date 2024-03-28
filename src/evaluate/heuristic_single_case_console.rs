use crate::{
    circle::Circle,
    packing::{find_answer, is_valid_pack},
    ralgo::{
        dichotomy_step_ralgo::dichotomy_step_ralgo_result_with_iterations,
        ralgo_params::RalgoParams,
    }, utils::FloatType,
};

pub fn heuristic_single_case_console(
    test_number: u32,
    algorithm_params: &[(bool, FloatType)],
    alpha_q1_pairs: &Vec<(FloatType, FloatType)>,
) -> (FloatType, Vec<Circle>) {
    let mut radiuses = (1..=test_number).map(|x| x as FloatType).collect::<Vec<_>>();
    let (main_circle_radius, circles) = find_answer(&mut radiuses, 10000);

    let mut answer_main_circle_radius = main_circle_radius;
    let mut answer_circles = circles.clone();

    for (alpha, q1) in alpha_q1_pairs {
        for (reset_step, eps) in algorithm_params {
            let ralgo_params = RalgoParams::default()
                .with_alpha(*alpha)
                .with_q1(*q1)
                .with_max_iterations(100_000);

            let ralgo_results = dichotomy_step_ralgo_result_with_iterations(
                main_circle_radius,
                &circles,
                *reset_step,
                *eps,
                &ralgo_params,
            );
            let new_main_circle_radius = ralgo_results.main_circle_radius;
            let new_circles = ralgo_results.circles;

            if is_valid_pack(new_main_circle_radius, &new_circles)
                && new_main_circle_radius < answer_main_circle_radius
            {
                answer_main_circle_radius = new_main_circle_radius;
                answer_circles = new_circles;
            }
        }
    }

    (answer_main_circle_radius, answer_circles)
}
