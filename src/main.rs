use crate::{
    evaluate::heuristic_all_cases::heuristic_all_cases, evaluate::random_all_cases::random_all_cases, utils::measure_time,
};

mod circle;
mod evaluate;
mod packing;
mod plot;
mod point;
mod ralgo;
mod utils;

fn main() {
    let ralgo_params = [
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

    let (total_time_of_random, _) = measure_time(|| random_all_cases(&ralgo_params, 0.7403));
    println!("Total time (random): {}", total_time_of_random);

    let (total_time_of_heuristic, _) = measure_time(|| heuristic_all_cases(&ralgo_params));
    println!("Total time (heuristic): {}", total_time_of_heuristic);
}
