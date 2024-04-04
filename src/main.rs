use utils::FloatType;

use crate::{
    evaluate::{
        heuristic_all_cases::heuristic_all_cases, heuristic_single_case::heuristic_single_case,
        heuristic_single_case_console::heuristic_single_case_console,
        random_all_cases::random_all_cases, random_single_case_console::random_single_case_console,
        random_single_case_iterations::random_single_case_iterations,
    },
    ralgo::ralgo_params::RalgoParams,
    utils::measure_time,
};

mod circle;
mod evaluate;
mod packing;
mod packomania;
mod plot;
mod point;
mod ralgo;
mod read_and_gen_tables;
mod utils;

fn main() {
    let variants_array = [false, true];
    let eps_array = [0.0];
    let algorithm_params = eps_array
        .iter()
        .flat_map(|eps| variants_array.iter().map(|variant| (*variant, *eps)))
        .collect::<Vec<(bool, FloatType)>>();

    let alpha_array = [1.5, 2.0, 2.5];
    let q1_array = [0.8, 0.85, 0.9, 0.95, 1.0];
    let alpha_q1_pairs = alpha_array
        .iter()
        .flat_map(|alpha| q1_array.iter().map(|q1| (*alpha, *q1)))
        .collect::<Vec<(FloatType, FloatType)>>();

    let ralgo_params = RalgoParams::default().with_max_iterations(15_000);

    random_single_case_iterations(50, 50, &algorithm_params, &ralgo_params, &alpha_q1_pairs).ok();

    read_and_gen_tables::read_and_gen_random_single_case_iterations(
        50,
        &variants_array,
        &eps_array,
    )
    .unwrap();
}
