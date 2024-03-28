use super::utils::{calculate_points, get_input_data, get_jury_answer};
use crate::{
    circle::Circle,
    packing,
    point::Point,
    ralgo::{
        dichotomy_step_ralgo::dichotomy_step_ralgo_result_with_iterations,
        ralgo_params::RalgoParams,
    }, utils::FloatType,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rust_xlsxwriter::{column_number_to_name, Format, Formula, Workbook, Worksheet};
use std::{
    io::{self},
    sync::{Arc, Mutex},
};

const HEADING_NAMES: [&str; 6] = [
    "R",
    "Points",
    "Is valid?",
    "ralgo_calls",
    "Iterations",
    "calcfg_calls",
];

fn get_table_headings(params: &[(bool, FloatType)]) -> Vec<String> {
    let mut headings: Vec<String> = vec!["Launch", "R_gen", "R", "r"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for (reset, eps) in params {
        let reset_str = if *reset { "P" } else { "B" };
        for i in 0..HEADING_NAMES.len() {
            headings.push(format!("{} {} EPS={}", &HEADING_NAMES[i], reset_str, eps));
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

        let (mut x, mut y);
        loop {
            (x, y) = (
                rng.gen_range(-main_circle_radius..=main_circle_radius),
                rng.gen_range(-main_circle_radius..=main_circle_radius),
            );

            if x.powi(2) + y.powi(2) <= main_circle_radius.powi(2) {
                break;
            }
        }

        circles.push(Circle::new(radiuses[i], Point { x, y }))
    }

    let mut r = FloatType::MAX;

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

fn get_updated_main_cirlce_radius(circles: &Vec<Circle>, r: FloatType) -> FloatType {
    return circles
        .iter()
        .map(|c| (c.center.unwrap().x.powi(2) + c.center.unwrap().y.powi(2)).sqrt() + r)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
}

pub fn write_row_block(
    worksheet: &Arc<Mutex<&mut Worksheet>>,
    row: u32,
    col: u16,
    main_circle_radius: FloatType,
    is_valid: bool,
    points: FloatType,
    ralgo_calls: u32,
    iterations: u32,
    calcfg_calls: u32,
    format: &Format,
) {
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col, main_circle_radius, &format)
        .ok();
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col + 1, points, &format)
        .ok();
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col + 2, is_valid, &format)
        .ok();
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col + 3, ralgo_calls, &format)
        .ok();
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col + 4, iterations, &format)
        .ok();
    worksheet
        .lock()
        .unwrap()
        .write_with_format(row, col + 5, calcfg_calls, &format)
        .ok();
}

pub fn random_single_case_iterations(
    test_number: usize,
    launches: usize,
    algorithm_params: &[(bool, FloatType)],
    ralgo_params: &RalgoParams,
    alpha_q1_pairs: Vec<(FloatType, FloatType)>,
) -> io::Result<()> {
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(0)));
    let (_, radiuses) = get_input_data(test_number as u32);
    let jury_answer = get_jury_answer(test_number as u32);

    let gen_main_circle_radius: FloatType = radiuses.iter().map(|r| r.powi(2)).sum::<FloatType>().sqrt() * 1.2;

    let mut workbook: Workbook = Workbook::new();

    for (alpha, q1) in alpha_q1_pairs {
        let ralgo_params = ralgo_params.with_alpha(alpha).with_q1(q1);
        println!("Generate worksheet with ralgo params = {ralgo_params:?}");

        let worksheet = Arc::new(Mutex::new(workbook.add_worksheet()));

        worksheet
            .lock()
            .unwrap()
            .set_name(format!("alpha = {alpha}, q1 = {q1}"))
            .ok();

        let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

        // setup headings
        for (col, data) in get_table_headings(algorithm_params).iter().enumerate() {
            worksheet
                .lock()
                .unwrap()
                .write_with_format(0, col as u16, data, &cell_format)
                .ok();
        }

        (1..=launches).into_par_iter().for_each(|launch| {
            println!("Launch: {launch}");

            let worksheet = Arc::clone(&worksheet);
            let rng = Arc::clone(&rng);

            worksheet
                .lock()
                .unwrap()
                .write_with_format(launch as u32, 0, launch as u32, &cell_format)
                .ok();

            let (circles, r) = generate_random_arrangement(gen_main_circle_radius, &rng, &radiuses);
            let updated_main_circle_radius = get_updated_main_cirlce_radius(&circles, r);

            worksheet
                .lock()
                .unwrap()
                .write_with_format(launch as u32, 1, gen_main_circle_radius, &cell_format)
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(launch as u32, 2, updated_main_circle_radius, &cell_format)
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(launch as u32, 3, r, &cell_format)
                .ok();

            for (index, (reset_step, eps)) in algorithm_params.iter().enumerate() {
                // get result of dichotomy algorithm
                let ralgo_result = dichotomy_step_ralgo_result_with_iterations(
                    updated_main_circle_radius,
                    &circles,
                    *reset_step,
                    *eps,
                    &ralgo_params,
                );
                let points = calculate_points(ralgo_result.main_circle_radius, jury_answer);

                // write dichotomy results into table
                write_row_block(
                    &worksheet,
                    launch as u32,
                    (index * HEADING_NAMES.len() + 4) as u16,
                    ralgo_result.main_circle_radius,
                    packing::is_valid_pack(ralgo_result.main_circle_radius, &ralgo_result.circles),
                    points,
                    ralgo_result.ralgo_calls,
                    ralgo_result.iterations,
                    ralgo_result.calcfg_calls,
                    &cell_format,
                );
            }
        });

        let (first_row_index, last_row_index) = (2, launches + 1);
        let generate_range = |column: String| -> String {
            format!("{column}{first_row_index}:{column}{last_row_index}")
        };

        let mut col = 4_u16;
        while col < (algorithm_params.len() * HEADING_NAMES.len() + 4) as u16 {
            let radius_column = column_number_to_name(col);
            let point_column = column_number_to_name(col + 1);
            let validation_column = column_number_to_name(col + 2);
            let ralgo_calls_column = column_number_to_name(col + 3);
            let iteration_column = column_number_to_name(col + 4);
            let calcfg_calls_column = column_number_to_name(col + 5);

            let radius_range = generate_range(radius_column);
            let point_range = generate_range(point_column);
            let validation_range = generate_range(validation_column);
            let ralgo_calls_range = generate_range(ralgo_calls_column);
            let iteration_range = generate_range(iteration_column);
            let calcfg_calls_range = generate_range(calcfg_calls_column);

            let best_result_row_formula = format!(
                "MATCH(MINIFS({radius_range}; {validation_range}; TRUE()); {radius_range}; 0)"
            );

            let best_result_radius_formula =
                format!("=INDEX({radius_range}; {best_result_row_formula}; 0)");

            let best_result_points_formula =
                format!("=INDEX({point_range}; {best_result_row_formula}; 0)");

            let best_result_ralgo_calls_formula =
                format!("=INDEX({ralgo_calls_range}; {best_result_row_formula}; 0)");

            let best_result_iterations_formula =
                format!("=INDEX({iteration_range}; {best_result_row_formula}; 0)");

            let best_result_calcfg_calls_formula =
                format!("=INDEX({calcfg_calls_range}; {best_result_row_formula}; 0)");

            worksheet
                .lock()
                .unwrap()
                .write_with_format(
                    last_row_index as u32,
                    col,
                    Formula::new(best_result_radius_formula),
                    &cell_format,
                )
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(
                    last_row_index as u32,
                    col + 1,
                    Formula::new(best_result_points_formula),
                    &cell_format,
                )
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(
                    last_row_index as u32,
                    col + 3,
                    Formula::new(best_result_ralgo_calls_formula),
                    &cell_format,
                )
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(
                    last_row_index as u32,
                    col + 4,
                    Formula::new(best_result_iterations_formula),
                    &cell_format,
                )
                .ok();

            worksheet
                .lock()
                .unwrap()
                .write_with_format(
                    last_row_index as u32,
                    col + 5,
                    Formula::new(best_result_calcfg_calls_formula),
                    &cell_format,
                )
                .ok();

            col += HEADING_NAMES.len() as u16;
        }

        worksheet.lock().unwrap().autofit();
    }

    workbook
        .save(format!(
            "./results/random/random-single-result-test-{test_number}.xlsx"
        ))
        .ok();

    Ok(())
}
