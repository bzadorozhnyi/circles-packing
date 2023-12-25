use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::evaluate::utils::*;
use crate::packing::{self};
use crate::ralgo::dichotomy_step_ralgo;
use crate::utils::measure_time;
use crate::{circle::Circle, point::Point};
use rust_xlsxwriter::{column_number_to_name, Format, Formula, Workbook};
use std::{fs, io, thread};
use std::{fs::File, io::BufReader};

fn get_table_headings(params: &[(bool, f32)]) -> Vec<String> {
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

fn get_optimal_random_arrangement(
    rng: &mut StdRng,
    launches_number: usize,
    main_circle_radius: f32,
    radiuses: &Vec<f32>,
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

pub fn random_all_cases(ralgo_params: &[(bool, f32)], density: f32) -> io::Result<()> {
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

    // get sorted path (in linux paths are unsorted)
    let mut sorted_paths = fs::read_dir("./input/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    sorted_paths.sort();

    let number_of_tests = sorted_paths.len();

    for (test_number, path) in sorted_paths.into_iter().enumerate() {
        println!("Test {}", test_number + 1);

        // write the test number in the far left column
        let row_number = (test_number + 1) as u32;
        worksheet.write(row_number, 0, row_number).ok();

        // get input data
        let file_name = path.display().to_string();
        let input_file = File::open(file_name).expect("Failed to open file");
        let reader = BufReader::new(input_file);
        let (_, radiuses) = get_input_data(reader);

        // generate start values
        let main_circle_radius: f32 =
            (radiuses.iter().map(|r| r.powi(2)).sum::<f32>() / density).sqrt();

        let circles = get_optimal_random_arrangement(&mut rng, 700, main_circle_radius, &radiuses);

        let main_circle_radius = main_circle_radius * 10.0;

        // get jury answer of current test
        let jury_answer = get_jury_answer(row_number);

        let mut handles = vec![];

        // run dichotomy ralgo with different parameters in threads
        for (reset_step, eps) in ralgo_params.to_owned() {
            let copied_circles = circles.clone();

            let handle = thread::spawn(move || {
                // get result of dichotomy algorithm
                let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
                    dichotomy_step_ralgo(main_circle_radius, &copied_circles, reset_step, eps)
                });
                let points = calculate_points(new_main_circle_radius, jury_answer);

                return (
                    new_main_circle_radius,
                    points,
                    packing::is_valid_pack(new_main_circle_radius, &new_circles),
                    ralgo_time,
                );
            });

            handles.push(handle);
        }

        // write dichotomy results into table
        for (index, handle) in handles.into_iter().enumerate() {
            let (radius, points, is_valid, time) = handle.join().unwrap();
            write_row_block(
                worksheet,
                row_number,
                (index * 4 + 1) as u16, // skip first 5 columns (heuristic result)
                radius,
                is_valid,
                points,
                time,
                &cell_format,
            );
        }
    }

    let mut col: u16 = 2;
    while col < (ralgo_params.len() * 4 + 1) as u16 {
        // number of columns
        let column: String = column_number_to_name(col);
        worksheet
            .write_with_format(
                (number_of_tests + 1) as u32,
                col,
                Formula::new(format!("=SUM({column}2:{column}{})", (number_of_tests + 1))),
                &cell_format,
            )
            .ok();
        col += 2;
    }

    worksheet.autofit();

    workbook
        .save(format!(
            "./results/random/random-result-multi (density = {density:.5}).xlsx"
        ))
        .ok();

    Ok(())
}
