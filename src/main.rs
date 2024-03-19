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

fn test_5_circles() {
    let radiuses = [1.0, 2.0, 3.0, 4.0, 5.0];
    let normalized_coordinates = vec![
        (
            0.713892571232417631221701695511,
            -0.344155848273873411585688583918,
        ),
        (
            -0.681865978095046812604637384133,
            0.164440032623038188646662065278,
        ),
        (
            0.646305330827477461038211041777,
            0.163715852151450969802426641891,
        ),
        (
            -0.025724911869879508301792021048,
            0.555028729847845797365923205380,
        ),
        (
            -0.001705619441145294635938306110,
            -0.444527439510283851885140364387,
        ),
    ];
    let r = 9.001397746050;

    let coordinates: Vec<(f64, f64)> = normalized_coordinates
        .iter()
        .map(|(x, y)| (x * r, y * r))
        .collect();

    for i in 0..5 {
        let (x, y) = coordinates[i];
        let temp = x.powi(2) + y.powi(2) - (r - radiuses[i]).powi(2);
        println!("Circle: i = {}, {temp:1.e}", i + 1);
    }

    for i in 0..5 {
        let (xi, yi) = coordinates[i];
        for j in (i + 1)..5 {
            let (xj, yj) = coordinates[j];

            let temp = -(xi - xj).powi(2) - (yi - yj).powi(2) + (radiuses[i] + radiuses[j]).powi(2);
            println!("Circles: i = {}, j = {}, {temp:1.e}", i + 1, j + 1);
        }
    }
}

fn main() {
    test_5_circles();
    // let eps_array = [0.0];
    // let variants_array = [false, true];
    // let algorithm_params = eps_array
    //     .iter()
    //     .flat_map(|eps| variants_array.iter().map(|variant| (*variant, *eps)))
    //     .collect::<Vec<(bool, f64)>>();

    // let ralgo_params = RalgoParams::default().with_max_iterations(50_000);
    // let alpha_array = [1.5, 2.0, 2.5];
    // let q1_array = [0.8, 0.85, 0.9, 0.95, 1.0];
    // let alpha_q1_pairs = alpha_array
    //     .iter()
    //     .flat_map(|alpha| q1_array.iter().map(|q1| (*alpha, *q1)))
    //     .collect::<Vec<(f64, f64)>>();

    // // let (total_time_of_random, _) =
    // //     measure_time(|| random_all_cases(&algorithm_params, 0.7403, &ralgo_params));
    // // println!("Total time (random): {}", total_time_of_random);

    // for (alpha, q1) in &alpha_q1_pairs {
    //     println!("alpha = {alpha}, q1 = {q1}");
    //     let ralgo_params = ralgo_params.with_alpha(*alpha).with_q1(*q1);
    //     let (total_time_of_heuristic, _) =
    //         measure_time(|| heuristic_all_cases(&algorithm_params, &ralgo_params));
    //     println!(
    //         "Total time (heuristic): {}, alpha = {}, q1 = {}",
    //         total_time_of_heuristic, alpha, q1
    //     );
    // }

    // read_and_gen_tables::read_and_gen_heuristic(&alpha_q1_pairs).unwrap();
}
