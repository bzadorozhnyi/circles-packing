use crate::{
    evaluate::{
        heuristic_all_cases::heuristic_all_cases, heuristic_single_case::heuristic_single_case,
        random_all_cases::random_all_cases, random_single_case::random_single_case,
    },
    ralgo::ralgo_params::RalgoParams,
    utils::measure_time,
};

mod circle;
mod evaluate;
mod packing;
mod plot;
mod point;
mod ralgo;
mod utils;

fn main() {
    let algorithm_params = [
        (false, 1e-2),
        (true, 1e-2),
        (false, 1e-3),
        (true, 1e-3),
        (false, 1e-4),
        (true, 1e-4),
        (false, 1e-5),
        (true, 1e-5),
        (false, 0.0),
        (true, 0.0),
    ];

    let ralgo_params = RalgoParams::default();

    let (total_time_of_random, _) =
        measure_time(|| random_all_cases(&algorithm_params, 0.7403, &ralgo_params));
    println!("Total time (random): {}", total_time_of_random);

    let (total_time_of_heuristic, _) =
        measure_time(|| heuristic_all_cases(&algorithm_params, &ralgo_params));
    println!("Total time (heuristic): {}", total_time_of_heuristic);

    let test_number = 10;
    let (total_time_of_single_random, _) =
        measure_time(|| random_single_case(test_number, 50, &algorithm_params, &ralgo_params));
    println!(
        "Total time (single random test = {test_number}): {}",
        total_time_of_single_random
    );

    heuristic_single_case(12);
}
