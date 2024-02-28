use calamine::{open_workbook, DataType, Error, Range, Reader, Xlsx};
use rayon::range;
use regex::Regex;
use rust_xlsxwriter::{
    Chart, ChartFormat, ChartLegendPosition, ChartLine, ChartLineDashType, ChartPatternFill,
    ChartType, Color, Format, Workbook, Worksheet,
};

#[derive(Clone, Copy, Debug)]
struct BlockData {
    pub r_start: f64,
    pub r: f64,
    pub circle_radius: f64, // R
    pub points: f64,
    pub is_valid_packing: bool,
    pub ralgo_calls: u32,
    pub iterations: u32,
    pub calcfg_calls: u32,
}

impl BlockData {
    pub fn new(
        r_start: f64,
        r: f64,
        circle_radius: f64,
        points: f64,
        is_valid_packing: bool,
        ralgo_calls: u32,
        iterations: u32,
        calcfg_calls: u32,
    ) -> Self {
        BlockData {
            r_start,
            r,
            circle_radius,
            points,
            is_valid_packing,
            ralgo_calls,
            iterations,
            calcfg_calls,
        }
    }
}

fn add_chart(
    worksheet: &mut Worksheet,
    values_ranges: &[&String],
    legends: &[&String],
    categories: &[&String],
    range_colors: &[Color],
    x_axis_name: &String,
    y_axis_name: &String,
    row_position: u32,
    column_position: u16,
) {
    let mut chart = Chart::new(ChartType::Bar);

    chart.legend().set_position(ChartLegendPosition::Bottom);
    chart.x_axis().set_name(x_axis_name);
    chart.y_axis().set_name(y_axis_name);

    chart.set_width(1200).set_height(600);

    for i in 0..values_ranges.len() {
        chart
            .add_series()
            .set_format(
                ChartFormat::new().set_pattern_fill(
                    ChartPatternFill::new().set_background_color(range_colors[i]),
                ),
            )
            .set_name(legends[i])
            .set_values(values_ranges[i])
            .set_categories(categories[i]);
    }

    worksheet
        .insert_chart(row_position, column_position, &chart)
        .ok();
}

fn add_chart_line(
    worksheet: &mut Worksheet,
    values_ranges: &[&String],
    legends: &[&String],
    chart_formats: &mut Vec<&mut ChartFormat>,
    row_position: u32,
    column_position: u16,
) {
    let mut chart = Chart::new(ChartType::Line);

    chart.legend().set_position(ChartLegendPosition::Bottom);
    chart.x_axis().set_name("r");
    chart.y_axis().set_name("Бали");

    chart.set_width(1200).set_height(600);

    for i in 0..values_ranges.len() {
        chart
            .add_series()
            .set_values(values_ranges[i])
            .set_name(legends[i])
            .set_format(&mut chart_formats[i].clone())
            .set_categories(&format!("{}!$E$3:$E$32", worksheet.name()));
    }

    worksheet
        .insert_chart(row_position, column_position, &chart)
        .ok();
}

pub fn read_and_gen_random_single_case_iterations(
    test_number: usize,
    variants_array: &[bool],
    eps_array: &[f64],
) -> Result<(), Error> {
    let start_column = 4;
    let number_of_launches = 50;
    let block_size = 6_usize;
    let cell_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_num_format("0.#####");
    let better_result_cell_format = Format::new()
        .set_align(rust_xlsxwriter::FormatAlign::Center)
        .set_background_color(Color::RGB(0xaccc9f))
        .set_num_format("0.#####");

    let mut workbook: Xlsx<_> = open_workbook(format!(
        "results/random/random-single-result-test-{test_number}.xlsx"
    ))?;
    let mut output_workbook = Workbook::new();

    fn write_block(
        worksheet: &mut Worksheet,
        row: u32,
        col: u16,
        alpha: f64,
        q1: f64,
        eps: f64,
        result: &BlockData,
        cell_format: &Format,
    ) {
        worksheet
            .write_with_format(row, col, format!("{eps:.5}"), &cell_format)
            .ok();
        worksheet
            .write_with_format(
                row,
                col + 1,
                alpha,
                &cell_format.clone().set_num_format("0.00"),
            )
            .ok();
        worksheet
            .write_with_format(
                row,
                col + 2,
                q1,
                &cell_format.clone().set_num_format("0.00"),
            )
            .ok();
        worksheet
            .write_with_format(row, col + 3, result.r_start, &cell_format)
            .ok();
        worksheet
            .write_with_format(
                row,
                col + 4,
                format!("{:.2}", result.r).replacen(".", ",", 1),
                &cell_format,
            )
            .ok();
        worksheet
            .write_with_format(row, col + 5, result.circle_radius, &cell_format)
            .ok();
        worksheet
            .write_with_format(
                row,
                col + 6,
                result.points,
                &cell_format.clone().set_num_format("0.00"),
            )
            .ok();
        worksheet
            .write_with_format(row, col + 7, result.is_valid_packing, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 8, result.ralgo_calls, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 9, result.iterations, &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 10, result.calcfg_calls, &cell_format)
            .ok();
        // calcfg_calls / iterations ratio
        worksheet
            .write_with_format(
                row,
                col + 11,
                (result.calcfg_calls as f64) / (result.iterations as f64),
                &cell_format.clone().set_num_format("0.00"),
            )
            .ok();
        // iterations / ralgo_calls
        worksheet
            .write_with_format(
                row,
                col + 12,
                (result.iterations as f64) / (result.ralgo_calls as f64),
                &cell_format.clone().set_num_format("0.00"),
            )
            .ok();

        worksheet
            .write_with_format(row, 100, format!("{alpha} / {q1}"), &cell_format)
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
            .write_with_format(row, col + 3, "R_start", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 4, "r", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 5, "R", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 6, "Points", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 7, "is_valid?", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 8, "nralg", &cell_format)
            .ok(); // ralgo_calls
        worksheet
            .write_with_format(row, col + 9, "itn", &cell_format)
            .ok(); // iterations
        worksheet
            .write_with_format(row, col + 10, "nfg", &cell_format)
            .ok(); // calcfg_calls
        worksheet
            .write_with_format(row, col + 11, "nfg / itn", &cell_format)
            .ok();
        worksheet
            .write_with_format(row, col + 12, "itn / nralg", &cell_format)
            .ok();
    }

    fn get_alpha_q1(sheet_name: &String) -> Option<(f64, f64)> {
        let pattern = r#"alpha = (\d.*), q1 = (\d.*)"#;
        let regex = Regex::new(pattern).unwrap();

        if let Some(captures) = regex.captures(&sheet_name) {
            let alpha = captures.get(1).unwrap().as_str();
            let q1 = captures.get(2).unwrap().as_str();

            return Some((alpha.parse::<f64>().unwrap(), q1.parse::<f64>().unwrap()));
        } else {
            None
        }
    }

    for (eps_index, eps) in eps_array.iter().enumerate() {
        let eps_sheet_name = format!("EPS = {eps:.5}");
        println!("{eps_sheet_name}");

        let (mut first_variant_row, first_variant_col) = (1, 0);
        let (mut second_variant_row, second_variant_col) = (1, (block_size + 8) as u16);

        let output_worksheet = output_workbook
            .add_worksheet()
            .set_name(&eps_sheet_name)
            .unwrap();

        output_worksheet
            .write_with_format(0, first_variant_col, "Варіант 1", &cell_format)
            .ok();
        output_worksheet
            .write_with_format(0, second_variant_col as u16, "Варіант 2", &cell_format)
            .ok();

        write_heading_block(output_worksheet, first_variant_row, first_variant_col);
        first_variant_row += 1;

        write_heading_block(output_worksheet, second_variant_row, second_variant_col);
        second_variant_row += 1;

        for sheet_name in workbook.sheet_names() {
            let (alpha, q1) = get_alpha_q1(&sheet_name).unwrap();

            if let Ok(range) = workbook.worksheet_range(&sheet_name) {
                println!("{}", sheet_name);

                let mut all_data = vec![];
                for (block_index, reset_step) in variants_array.iter().enumerate() {
                    // collect data from block
                    let current_block_first_column =
                        start_column + block_index * block_size + eps_index * 2 * block_size;
                    let mut data: Vec<BlockData> = vec![];
                    for row in 1..=number_of_launches {
                        let r_start = range.get((row, 2)).unwrap().get_float().unwrap() as f64;
                        let r = range.get((row, 3)).unwrap().get_float().unwrap() as f64;
                        let circle_radius = range
                            .get((row, current_block_first_column))
                            .unwrap()
                            .get_float()
                            .unwrap() as f64;
                        let points = range
                            .get((row, current_block_first_column + 1))
                            .unwrap()
                            .get_float()
                            .unwrap() as f64;
                        let is_valid_packing = range
                            .get((row, current_block_first_column + 2))
                            .unwrap()
                            .get_bool()
                            .unwrap();
                        let ralgo_calls = range
                            .get((row, current_block_first_column + 3))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap();
                        let iterations = range
                            .get((row, current_block_first_column + 4))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap();
                        let calcfg_calls = range
                            .get((row, current_block_first_column + 5))
                            .unwrap()
                            .to_string()
                            .parse::<u32>()
                            .unwrap();

                        data.push(BlockData::new(
                            r_start,
                            r,
                            circle_radius,
                            points,
                            is_valid_packing,
                            ralgo_calls,
                            iterations,
                            calcfg_calls,
                        ));
                    }

                    // find best result
                    let (mut best_result_index, mut best_result) = (0, data[0].circle_radius);

                    for (index, row) in data.iter().enumerate() {
                        if row.is_valid_packing == true && best_result > row.circle_radius {
                            best_result_index = index;
                            best_result = row.circle_radius;
                        }
                    }

                    all_data.push((reset_step, eps, data[best_result_index]));
                }

                for (reset_step, current_eps, data) in &all_data {
                    if eps != *current_eps {
                        continue;
                    }

                    let another_variant_answer =
                        &all_data.iter().find(|(reset_step_2, eps_2, data_2)| {
                            reset_step != reset_step_2
                                && current_eps == eps_2
                                && data.circle_radius > data_2.circle_radius
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
                            **current_eps,
                            data,
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
                            **current_eps,
                            data,
                            current_cell_format,
                        );

                        first_variant_row += 1;
                    }
                }
            }
        }

        let x_axis_name = "Бали".to_string();
        let y_axis_name = "alpha / q1".to_string();

        add_chart(
            output_worksheet,
            &[&format!("{}!$G$3:$G$32", &eps_sheet_name)],
            &[&"Варіант 1".to_string()],
            &[&format!("{}!$CW$3:$CW$32", output_worksheet.name())],
            &[Color::Green],
            &x_axis_name,
            &y_axis_name,
            33,
            0,
        );
        add_chart(
            output_worksheet,
            &[&format!("{}!$U$3:$U$32", &eps_sheet_name)],
            &[&"Варіант 2".to_string()],
            &[&format!("{}!$CW$3:$CW$32", output_worksheet.name())],
            &[Color::Red],
            &x_axis_name,
            &y_axis_name,
            33,
            14,
        );
        add_chart(
            output_worksheet,
            &[
                &format!("{}!$G$3:$G$32", &eps_sheet_name),
                &format!("{}!$U$3:$U$32", &eps_sheet_name),
            ],
            &[&"Варіант 1".to_string(), &"Варіант 2".to_string()],
            &[
                &format!("{}!$CW$3:$CW$32", output_worksheet.name()),
                &format!("{}!$CW$3:$CW$32", output_worksheet.name()),
            ],
            &[Color::Green, Color::Red],
            &x_axis_name,
            &y_axis_name,
            66,
            0,
        );

        let format1 = ChartFormat::new()
            .set_line(
                ChartLine::new()
                    .set_color(Color::Green)
                    .set_transparency(50),
            )
            .clone();

        let format2 = ChartFormat::new()
            .set_line(
                ChartLine::new()
                    .set_color(Color::Blue)
                    .set_dash_type(ChartLineDashType::SquareDot)
                    .set_transparency(50),
            )
            .clone();

        add_chart_line(
            output_worksheet,
            &[&format!("{}!$G$3:$G$32", &eps_sheet_name)],
            &[&"Варіант 1".to_string()],
            &mut vec![&mut format1.clone()],
            99,
            0,
        );
        add_chart_line(
            output_worksheet,
            &[&format!("{}!$U$3:$U$32", &eps_sheet_name)],
            &[&"Варіант 2".to_string()],
            &mut vec![&mut format2.clone()],
            99,
            14,
        );
        add_chart_line(
            output_worksheet,
            &[
                &format!("{}!$G$3:$G$32", &eps_sheet_name),
                &format!("{}!$U$3:$U$32", &eps_sheet_name),
            ],
            &[&"Варіант 1".to_string(), &"Варіант 2".to_string()],
            &mut vec![&mut format1.clone(), &mut format2.clone()],
            132,
            0,
        );

        let x_axis_name = "Кількість ітерацій".to_string();
        let y_axis_name = "Бали".to_string();

        add_chart(
            output_worksheet,
            &[&format!("{}!$J$3:$J$32", output_worksheet.name())],
            &[&"Варіант 1".to_string()],
            &[&format!("{}!$G$3:$G$32", &eps_sheet_name)],
            &[Color::Green],
            &x_axis_name,
            &y_axis_name,
            165,
            0,
        );
        add_chart(
            output_worksheet,
            &[&format!("{}!$X$3:$X$32", output_worksheet.name())],
            &[&"Варіант 2".to_string()],
            &[&format!("{}!$U$3:$U$32", &eps_sheet_name)],
            &[Color::Red],
            &x_axis_name,
            &y_axis_name,
            165,
            14,
        );

        add_chart(
            output_worksheet,
            &[
                &format!("{}!$J$3:$J$32", output_worksheet.name()),
                &format!("{}!$X$3:$X$32", output_worksheet.name()),
            ],
            &[&"Варіант 1".to_string(), &"Варіант 2".to_string()],
            &[
                &format!("{}!$G$3:$G$32", &eps_sheet_name),
                &format!("{}!$U$3:$U$32", &eps_sheet_name),
            ],
            &[Color::Green, Color::Red],
            &x_axis_name,
            &y_axis_name,
            198,
            0,
        );

        output_worksheet.autofit();
    }

    output_workbook
        .save(format!(
            "./results/random/total result random-single-result-test-{test_number}-t.xlsx"
        ))
        .ok();

    Ok(())
}

pub fn read_and_gen_heuristic(alpha_q1_pairs: &Vec<(f64, f64)>) -> Result<(), Error> {
    let mut output_workbook = Workbook::new();
    let output_worksheet = output_workbook.add_worksheet();

    output_worksheet.write(0, 0, "alpha").ok();
    output_worksheet.write(0, 1, "q1").ok();
    output_worksheet.write(0, 2, "points_variant_1").ok();
    output_worksheet.write(0, 3, "iterations_variant_1").ok();
    output_worksheet.write(0, 4, "points_variant_2").ok();
    output_worksheet.write(0, 5, "iterations_variant_2").ok();

    for (table_row, (alpha, q1)) in alpha_q1_pairs.iter().enumerate() {
        let mut workbook: Xlsx<_> = open_workbook(format!(
            "results/heuristic/result-multi-alpha={alpha}-q1={q1}.xlsx"
        ))?;

        let sum_on_range = |range: &Range<DataType>,
                            row_start: usize,
                            col_start: usize,
                            row_end: usize,
                            col_end: usize| {
            let mut sum = 0.0;
            for row in row_start..=row_end {
                for col in col_start..=col_end {
                    sum += range.get((row, col)).unwrap().as_f64().unwrap();
                }
            }

            sum
        };

        if let Ok(range) = workbook.worksheet_range("Sheet1") {
            let points_variant_1 = sum_on_range(&range, 1, 6, 50, 6);
            let iterations_variant_1 = sum_on_range(&range, 1, 8, 50, 8) as u32;

            let points_variant_2 = sum_on_range(&range, 1, 10, 50, 10);
            let iterations_variant_2 = sum_on_range(&range, 1, 12, 50, 12) as u32;

            println!("p1 = {points_variant_1}, iter1 = {iterations_variant_1}");
            println!("p2 = {points_variant_2}, iter2 = {iterations_variant_2}");
            println!();

            output_worksheet.write(table_row as u32 + 1, 0, *alpha).ok();
            output_worksheet.write(table_row as u32 + 1, 1, *q1).ok();
            output_worksheet
                .write(table_row as u32 + 1, 2, points_variant_1)
                .ok();
            output_worksheet
                .write(table_row as u32 + 1, 3, iterations_variant_1)
                .ok();
            output_worksheet
                .write(table_row as u32 + 1, 4, points_variant_2)
                .ok();
            output_worksheet
                .write(table_row as u32 + 1, 5, iterations_variant_2)
                .ok();
        }
    }

    output_workbook
        .save(format!("./results/heuristic/total-heuristic.xlsx"))
        .ok();

    Ok(())
}
