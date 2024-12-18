use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::evaluate::utils::*;
use crate::packing::{self};
use crate::ralgo::dichotomy_step_ralgo::dichotomy_step_ralgo;
use crate::ralgo::ralgo_params::RalgoParams;
use crate::utils::{measure_time, FloatType};
use crate::{circle::Circle, point::Point};
use rust_xlsxwriter::{column_number_to_name, Format, Formula, Workbook};
use std::sync::{Arc, Mutex};
use std::{fs, io};

fn get_table_headings(params: &[(bool, FloatType)]) -> Vec<String> {
    let mut headings: Vec<String> = vec!["Test".to_string()];
    let heading_names = ["R", "Points", "Is valid?", "Time"];
    for (reset, eps) in params {
        let reset_str = if *reset { "P" } else { "B" };
        for i in 0..4 {
            headings.push(format!("{} {} EPS={}", &heading_names[i], reset_str, eps));
        }
    }

    return headings;
}

fn generate_random_arrangement(
    main_circle_radius: FloatType,
    rng: &Arc<Mutex<StdRng>>,
    radiuses: &Vec<FloatType>,
) -> (Vec<Circle>, FloatType) {
    let mut circles = vec![];
    for i in 0..radiuses.len() {
        let mut rng = rng.lock().unwrap();

        circles.push(Circle::new(
            radiuses[i],
            Point {
                x: rng.gen_range(-main_circle_radius..=main_circle_radius),
                y: rng.gen_range(-main_circle_radius..=main_circle_radius),
            },
        ))
    }

    let mut r = circles
        .iter()
        .map(|c| {
            let center = c.center.unwrap();
            main_circle_radius - (center.x.powi(2) + center.y.powi(2)).sqrt()
        })
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();

    for i in 0..circles.len() {
        let center_i = circles[i].center.unwrap();

        for j in (i + 1)..circles.len() {
            let center_j = circles[j].center.unwrap();

            r = r.min(
                (center_i.x - center_j.x).powi(2)
                    + (center_i.y.powi(2) - center_j.y.powi(2)).sqrt() / 2.0,
            );
        }
    }

    return (circles, r);
}

fn get_optimal_random_arrangement(
    rng: &Arc<Mutex<StdRng>>,
    launches_number: usize,
    main_circle_radius: FloatType,
    radiuses: &Vec<FloatType>,
) -> Vec<Circle> {
    let max_radius = *(radiuses
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap());

    let (mut optimal_circles, mut r) =
        generate_random_arrangement(main_circle_radius, rng, radiuses);

    for _ in 0..launches_number {
        let (new_circles, new_r) = generate_random_arrangement(main_circle_radius, rng, radiuses);

        if new_r >= max_radius && new_r >= r {
            (optimal_circles, r) = (new_circles, new_r);
        }
    }

    return optimal_circles;
}

pub fn random_all_cases(
    algorithm_params: &[(bool, FloatType)],
    density: FloatType,
    ralgo_params: &RalgoParams,
) -> io::Result<()> {
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(0)));

    let mut workbook: Workbook = Workbook::new();
    let worksheet = Arc::new(Mutex::new(workbook.add_worksheet()));
    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

    // setup headings
    for (col, data) in get_table_headings(algorithm_params).iter().enumerate() {
        worksheet
            .lock()
            .unwrap()
            .write_with_format(0, col as u16, data, &cell_format)
            .ok();
    }

    let number_of_tests = fs::read_dir("./input/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .len();

    (1..=50).into_par_iter().for_each(|test_number| {
        println!("Test {}", test_number);
        let rng = Arc::clone(&rng);

        // write the test number in the far left column
        worksheet
            .lock()
            .unwrap()
            .write(test_number, 0, test_number)
            .ok();

        let (_, radiuses) = get_input_data(test_number);
        let jury_answer = get_jury_answer(test_number);

        // generate start values
        let main_circle_radius: FloatType =
            (radiuses.iter().map(|r| r.powi(2)).sum::<FloatType>() / density).sqrt();
        let circles = get_optimal_random_arrangement(&rng, 700, main_circle_radius, &radiuses);
        let main_circle_radius = main_circle_radius * 10.0;

        // run dichotomy ralgo with different parameters in threads
        for (index, (reset_step, eps)) in algorithm_params.iter().enumerate() {
            // get result of dichotomy algorithm
            let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
                dichotomy_step_ralgo(
                    main_circle_radius,
                    &circles,
                    *reset_step,
                    *eps,
                    ralgo_params,
                )
            });
            let points = calculate_points(new_main_circle_radius, jury_answer);

            // write dichotomy results into table
            write_row_block(
                &worksheet,
                test_number,
                (index * 4 + 1) as u16, // skip first 5 columns (heuristic result)
                new_main_circle_radius,
                packing::is_valid_pack(new_main_circle_radius, &new_circles),
                points,
                ralgo_time,
                &cell_format,
            );
        }
    });

    let mut col: u16 = 2;
    while col < (algorithm_params.len() * 4 + 1) as u16 {
        // number of columns
        let column: String = column_number_to_name(col);
        worksheet
            .lock()
            .unwrap()
            .write_with_format(
                (number_of_tests + 1) as u32,
                col,
                Formula::new(format!("=SUM({column}2:{column}{})", (number_of_tests + 1))),
                &cell_format,
            )
            .ok();
        col += 2;
    }

    worksheet.lock().unwrap().autofit();

    workbook
        .save(format!(
            "./results/random/random-result-multi (density = {density:.5}).xlsx"
        ))
        .ok();

    Ok(())
}
