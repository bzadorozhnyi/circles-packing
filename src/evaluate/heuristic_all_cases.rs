use crate::evaluate::utils::*;
use crate::packing::{self, find_answer};
use crate::ralgo::ralgo::dichotomy_step_ralgo;
use crate::ralgo::ralgo_params::RalgoParams;
use crate::utils::measure_time;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rust_xlsxwriter::{column_number_to_name, Format, Formula, Workbook};
use std::sync::{Arc, Mutex};
use std::{fs, io};

fn get_table_headings(params: &[(bool, f32)]) -> Vec<String> {
    let mut headings: Vec<String> = vec!["Test", "R", "Points", "Is valid?", "Time"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for (reset, eps) in params {
        let reset_str = if *reset { "P" } else { "B" };
        for i in 1..5 {
            headings.push(format!("{} {} EPS={}", &headings[i], reset_str, eps));
        }
    }

    return headings;
}

pub fn heuristic_all_cases(
    algorithm_params: &[(bool, f32)],
    ralgo_params: &RalgoParams,
) -> io::Result<()> {
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

    (1..=number_of_tests as u32)
        .into_par_iter()
        .for_each(|test_number| {
            println!("Test {}", test_number);

            // write the test number in the far left column
            worksheet
                .lock()
                .unwrap()
                .write(test_number, 0, test_number)
                .ok();

            let (_, mut radiuses) = get_input_data(test_number);
            let jury_answer = get_jury_answer(test_number);

            // get result of heuristic algorithm
            let (heuristic_time, (main_circle_radius, circles)) =
                measure_time(|| find_answer(&mut radiuses, 100));

            let points = calculate_points(main_circle_radius, jury_answer);

            // fill in table heuristic packing result
            write_row_block(
                &worksheet,
                test_number,
                1,
                main_circle_radius,
                packing::is_valid_pack(main_circle_radius, &circles),
                points,
                heuristic_time,
                &cell_format,
            );

            // run dichotomy ralgo with different parameters in threads
            for (index, (reset_step, eps)) in algorithm_params.iter().enumerate() {
                // get result of dichotomy algorithm
                let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
                    dichotomy_step_ralgo(
                        main_circle_radius,
                        &&circles,
                        *reset_step,
                        *eps,
                        &ralgo_params,
                    )
                });
                let points = calculate_points(new_main_circle_radius, jury_answer);

                // write dichotomy results into table
                write_row_block(
                    &worksheet,
                    test_number,
                    (index * 4 + 5) as u16, // skip first 5 columns (heuristic result)
                    new_main_circle_radius,
                    packing::is_valid_pack(new_main_circle_radius, &new_circles),
                    points,
                    ralgo_time,
                    &cell_format,
                );
            }
        });

    let mut col: u16 = 2;
    while col < (algorithm_params.len() * 4 + 5) as u16 {
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
        .save("./results/heuristic/result-multi-linux.xlsx")
        .ok();

    Ok(())
}
