use std::{
    fs::File,
    io::{self, BufReader},
};

use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_xlsxwriter::{Format, Workbook};

use crate::{
    circle::Circle, packing, point::Point, ralgo::dichotomy_step_ralgo, utils::measure_time,
};

use super::utils::{calculate_points, get_input_data, get_jury_answer, write_row_block};

fn get_table_headings(params: &[(bool, f32)]) -> Vec<String> {
    let mut headings: Vec<String> = vec!["Launch".to_string()];
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
    main_circle_radius: f32,
    rng: &mut StdRng,
    radiuses: &Vec<f32>,
) -> (Vec<Circle>, f32) {
    let mut circles = vec![];
    for i in 0..radiuses.len() {
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

fn get_updated_main_cirlce_radius(circles: &Vec<Circle>, r: f32) -> f32 {
    return circles
        .iter()
        .map(|c| (c.center.unwrap().x.powi(2) + c.center.unwrap().y.powi(2)).sqrt() + r)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
}

pub fn random_single_case(
    test_number: usize,
    launches: usize,
    ralgo_params: &[(bool, f32)],
) -> io::Result<()> {
    let mut rng = StdRng::seed_from_u64(0);

    let mut workbook: Workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

    // setup headings
    for (col, data) in get_table_headings(ralgo_params).iter().enumerate() {
        worksheet
            .write_with_format(0, col as u16, data, &cell_format)
            .ok();
    }

    // get input data
    let file_name = format!("./input/input{:03}.txt", test_number);
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);
    let (_, radiuses) = get_input_data(reader);

    // get jury answer of current test
    let jury_answer = get_jury_answer(test_number as u32);

    let main_circle_radius: f32 = radiuses.iter().map(|r| r.powi(2)).sum::<f32>().sqrt();

    for iter in 1..=launches {
        worksheet
            .write_with_format(iter as u32, 0, iter as u32, &cell_format)
            .ok();

        let (circles, r) = generate_random_arrangement(main_circle_radius, &mut rng, &radiuses);

        let updated_main_circle_radius = get_updated_main_cirlce_radius(&circles, r) * 10.0;

        // let mut results = vec![];
        let results: Vec<(f32, f32, bool, f32)> = ralgo_params
            .par_iter()
            .map(|(reset_step, eps)| {
                // get result of dichotomy algorithm
                let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
                    dichotomy_step_ralgo(updated_main_circle_radius, &circles, *reset_step, *eps)
                });
                let points = calculate_points(new_main_circle_radius, jury_answer);

                (
                    new_main_circle_radius,
                    points,
                    packing::is_valid_pack(new_main_circle_radius, &new_circles),
                    ralgo_time,
                )
            })
            .collect();

        // for (reset_step, eps) in ralgo_params {
        //     // get result of dichotomy algorithm
        //     let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
        //         dichotomy_step_ralgo(updated_main_circle_radius, &circles, *reset_step, *eps)
        //     });
        //     let points = calculate_points(new_main_circle_radius, jury_answer);

        //     results.push((
        //         new_main_circle_radius,
        //         points,
        //         packing::is_valid_pack(new_main_circle_radius, &new_circles),
        //         ralgo_time,
        //     ));
        // }

        // write dichotomy results into table
        for (index, result) in results.into_iter().enumerate() {
            let (radius, points, is_valid, time) = result;
            write_row_block(
                worksheet,
                iter as u32,
                (index * 4 + 1) as u16,
                radius,
                is_valid,
                points,
                time,
                &cell_format,
            );
        }
    }

    worksheet.autofit();

    workbook
        .save(format!(
            "./results/random/random-single-result-test-{test_number}.xlsx"
        ))
        .ok();

    Ok(())
}
