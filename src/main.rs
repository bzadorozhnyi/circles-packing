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

pub fn get_input_data(test_number: u32) -> (f64, Vec<f64>, Vec<(f64, f64)>) {
    use std::str::FromStr;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file_name = format!("./packomania/{test_number}.txt");
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().expect("Empty file").unwrap();
    let main_radius: f64 = first_line
        .trim()
        .parse()
        .expect("Invalid main circle radius");

    let mut radiuses: Vec<f64> = Vec::new();
    let mut normalized_coordinates: Vec<(f64, f64)> = Vec::new();
    for line in lines {
        let input_line = line.expect("Failed to read line");
        let mut iter = input_line.split_whitespace();

        let radius = f64::from_str(iter.next().unwrap()).unwrap();
        let x = f64::from_str(iter.next().unwrap()).unwrap();
        let y = f64::from_str(iter.next().unwrap()).unwrap();

        radiuses.push(radius);
        normalized_coordinates.push((x, y));
    }

    return (main_radius, radiuses, normalized_coordinates);
}

fn test_packomania_circles(test_number: u32) {
    let (main_radius, radiuses, normalized_coordinates) = get_input_data(test_number);

    let coordinates: Vec<(f64, f64)> = normalized_coordinates
        .iter()
        .map(|(x, y)| (x * main_radius, y * main_radius))
        .collect();

    for i in 0..5 {
        let (x, y) = coordinates[i];
        let temp = x.powi(2) + y.powi(2) - (main_radius - radiuses[i]).powi(2);
        if temp > 0.0 {
            println!("Circle: i = {}, {temp:1.e}", i + 1);
        }
    }

    for i in 0..5 {
        let (xi, yi) = coordinates[i];
        for j in (i + 1)..5 {
            let (xj, yj) = coordinates[j];

            let temp = -(xi - xj).powi(2) - (yi - yj).powi(2) + (radiuses[i] + radiuses[j]).powi(2);
            if temp > 0.0 {
                println!("Circles: i = {}, j = {}, {temp:1.e}", i + 1, j + 1);
            }
        }
    }
}

fn main() {
    test_packomania_circles(9);
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
