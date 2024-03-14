use crate::{
    evaluate::{
        heuristic_all_cases::heuristic_all_cases, heuristic_single_case::heuristic_single_case,
        random_all_cases::random_all_cases,
        random_single_case_iterations::random_single_case_iterations,
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
mod read_and_gen_tables;
mod utils;

fn main() {
    let eps_array = [0.0];
    let variants_array = [false, true];
    let algorithm_params = eps_array
        .iter()
        .flat_map(|eps| variants_array.iter().map(|variant| (*variant, *eps)))
        .collect::<Vec<(bool, f64)>>();

    let ralgo_params = RalgoParams::default().with_max_iterations(50_000);
    let alpha_array = [1.5, 2.0, 2.5];
    let q1_array = [0.8, 0.85, 0.9, 0.95, 1.0];
    let alpha_q1_pairs = alpha_array
        .iter()
        .flat_map(|alpha| q1_array.iter().map(|q1| (*alpha, *q1)))
        .collect::<Vec<(f64, f64)>>();

    // let (total_time_of_random, _) =
    //     measure_time(|| random_all_cases(&algorithm_params, 0.7403, &ralgo_params));
    // println!("Total time (random): {}", total_time_of_random);

    for (alpha, q1) in &alpha_q1_pairs {
        println!("alpha = {alpha}, q1 = {q1}");
        let ralgo_params = ralgo_params.with_alpha(*alpha).with_q1(*q1);
        let (total_time_of_heuristic, _) =
            measure_time(|| heuristic_all_cases(&algorithm_params, &ralgo_params));
        println!(
            "Total time (heuristic): {}, alpha = {}, q1 = {}",
            total_time_of_heuristic, alpha, q1
        );
    }

    read_and_gen_tables::read_and_gen_heuristic(&alpha_q1_pairs).unwrap();
}
