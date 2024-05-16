use packomania::find_best_heuristic;
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
    find_best_heuristic(10);
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

    let ralgo_params = RalgoParams::default()
        .with_alpha(1.5)
        .with_q1(1.0)
        .with_max_iterations(100_000);

    let (heuristic_all_cases_time, _) = measure_time(|| {
        heuristic_all_cases(&algorithm_params, &ralgo_params).unwrap();
    });
    println!("TIME: {heuristic_all_cases_time}");
}