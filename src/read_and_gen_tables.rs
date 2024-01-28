use calamine::{open_workbook, Error, Reader, Xlsx};
use regex::Regex;
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet};

pub fn read_and_gen_random_single_case_iterations(
    test_number: usize,
    algorithm_params: &[(bool, f32)],
) -> Result<(), Error> {
    let start_column = 4;
    let number_of_launches = 50;
    let block_size = 6_usize;
    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);
    let better_result_cell_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_background_color(Color::RGB(0xaccc9f));

    let (mut first_variant_row, first_variant_col) = (1, 0);
    let (mut second_variant_row, second_variant_col) = (1, (block_size + 6) as u16);

    let mut workbook: Xlsx<_> = open_workbook(format!(
        "results/random/random-single-result-test-{test_number}.xlsx"
    ))?;
    let mut output_workbook = Workbook::new();
    let output_worksheet = output_workbook.add_worksheet();

    output_worksheet
        .write_with_format(0, first_variant_col, "Варіант 1", &cell_format)
        .ok();
    output_worksheet
        .write_with_format(0, second_variant_col as u16, "Варіант 2", &cell_format)
        .ok();

    fn write_block(
        worksheet: &mut Worksheet,
        row: u32,
        col: u16,
        alpha: f32,
        q1: f32,
        eps: f32,
        result: (f32, f32, bool, u32, u32, u32),
        cell_format: &Format,
    ) {
        worksheet
            .write_with_format(row, col, format!("{eps:.5}"), &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 1, format!("{alpha:.2}"), &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 2, format!("{q1:.2}"), &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 3, format!("{:.5}", result.0), &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 4, format!("{:.2}", result.1), &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 5, result.2, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 6, result.3, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 7, result.4, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 8, result.5, &cell_format)
            .ok();
        // iterations / calcfg_calls ratio
        worksheet
            .write_with_format(
                row,
                col + 9,
                format!("{:.2}", (result.5 as f32) / (result.4 as f32)),
                &cell_format,
            )
            .ok();
        worksheet
            .write_with_format(
                row,
                col + 10,
                format!("{:.2}", (result.4 as f32) / (result.3 as f32)),
                &cell_format,
            )
            .ok();
    }

    fn write_heading_block(worksheet: &mut Worksheet, row: u32, col: u16) {
        let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

        worksheet
            .write_with_format(row, col, "EPS", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 1, "alpha", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 2, "q1", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 3, "R", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 4, "Points", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 5, "is_valid?", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 6, "nralg", &cell_format)
            .ok(); // ralgo_calls
        worksheet
            .write_with_format(row, col + 7, "itn", &cell_format)
            .ok(); // iterations
        worksheet
            .write_with_format(row, col + 8, "nfg", &cell_format)
            .ok(); // calcfg_calls
        worksheet
            .write_with_format(row, col + 9, "nfg / itn", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 10, "itn / nralg", &cell_format)
            .ok();
    }

    fn get_alpha_q1(sheet_name: &String) -> Option<(f32, f32)> {
        let pattern = r#"alpha = (\d.*), q1 = (\d.*)"#;
        let regex = Regex::new(pattern).unwrap();

        if let Some(captures) = regex.captures(&sheet_name) {
            let alpha = captures.get(1).unwrap().as_str();
            let q1 = captures.get(2).unwrap().as_str();

            return Some((alpha.parse::<f32>().unwrap(), q1.parse::<f32>().unwrap()));
        } else {
            None
        }
    }

    write_heading_block(output_worksheet, first_variant_row, first_variant_col);
    first_variant_row += 1;

    write_heading_block(output_worksheet, second_variant_row, second_variant_col);
    second_variant_row += 1;

    for sheet_name in workbook.sheet_names() {
        let (alpha, q1) = get_alpha_q1(&sheet_name).unwrap();

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            println!("{}", sheet_name);

            let mut all_data = vec![];
            for (block_index, (reset_step, eps)) in algorithm_params.iter().enumerate() {
                // collect data from block
                let current_block_first_column = start_column + block_index * block_size;
                let mut data: Vec<(f32, f32, bool, u32, u32, u32)> = vec![];
                for row in 1..=number_of_launches {
                    data.push((
                        range
                            .get((row, current_block_first_column))
                            .unwrap()
                            .get_float()
                            .unwrap() as f32,
                        range
                            .get((row, current_block_first_column + 1))
                            .unwrap()
                            .get_float()
                            .unwrap() as f32,
                        range
                            .get((row, current_block_first_column + 2))
                            .unwrap()
                            .get_bool()
                            .unwrap(),
                        range
                            .get((row, current_block_first_column + 3))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        range
                            .get((row, current_block_first_column + 4))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                        range
                            .get((row, current_block_first_column + 5))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap(),
                    ));
                }

                // find best result
                let (mut best_result_index, mut best_result) = (0, data[0].0);

                for (index, row) in data.iter().enumerate() {
                    if row.2 == true && best_result > row.0 {
                        best_result_index = index;
                        best_result = row.0;
                    }
                }

                all_data.push((reset_step, eps, data[best_result_index]));
            }

            for (reset_step, eps, data) in &all_data {
                let another_variant_answer =
                    &all_data.iter().find(|(reset_step_2, eps_2, data_2)| {
                        reset_step != reset_step_2 && eps == eps_2 && data.0 > data_2.0
                    });

                let current_cell_format = if another_variant_answer.is_none() {
                    &better_result_cell_format
                } else {
                    &cell_format
                };

                if **reset_step {
                    write_block(
                        output_worksheet,
                        second_variant_row,
                        second_variant_col,
                        alpha,
                        q1,
                        **eps,
                        *data,
                        current_cell_format,
                    );
                    second_variant_row += 1;
                } else {
                    write_block(
                        output_worksheet,
                        first_variant_row,
                        first_variant_col,
                        alpha,
                        q1,
                        **eps,
                        *data,
                        current_cell_format,
                    );

                    first_variant_row += 1;
                }

                // print
                println!("reset = {reset_step}, eps = {eps}, {:?}", data);
            }
        }
    }

    output_worksheet.autofit();

    output_workbook
        .save(format!(
            "./results/random/total result random-single-result-test-{test_number}.xlsx"
        ))
        .ok();

    Ok(())
}
