use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{circle::Circle, point::Point};

use crate::packing::{self};
use crate::ralgo::dichotomy_step_ralgo;
use crate::utils::measure_time;
use rust_xlsxwriter::{column_number_to_name, Format, Formula, Workbook, Worksheet};
use std::{fs, io, thread};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input_data(reader: BufReader<File>) -> (usize, Vec<f32>) {
    let mut lines = reader.lines();
    let first_line = lines.next().expect("Empty file").unwrap();
    let n: usize = first_line.trim().parse().expect("Invalid number");

    let mut radiuses: Vec<f32> = Vec::with_capacity(n);
    for line in lines {
        let radius: f32 = line
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Invalid number");

        radiuses.push(radius);
    }

    return (n, radiuses);
}

fn get_jury_answer(test_number: u32) -> f32 {
    let file_name = format!("./output/out{:03}.txt", test_number);
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .to_string()
        .parse::<f32>()
        .expect("Valid float jury answer.")
}

fn write_row_block(
    worksheet: &mut Worksheet,
    row: u32,
    col: u16,
    main_circle_radius: f32,
    is_valid: bool,
    points: f32,
    time: f32,
    format: &Format,
) {
    worksheet
        .write_with_format(row, col, main_circle_radius, &format)
        .ok();
    worksheet
        .write_with_format(row, col + 1, points, &format)
        .ok();
    worksheet
        .write_with_format(row, col + 2, is_valid, &format)
        .ok();
    worksheet
        .write_with_format(row, col + 3, time, &format)
        .ok();
}

fn calculate_points(answer: f32, jury_answer: f32) -> f32 {
    ((2.0 - (answer / jury_answer)) * 100.0).max(0.0).round()
}

fn get_table_headings(params: &[(bool, f32)]) -> Vec<String> {
    let mut headings: Vec<String> = vec![
        "Test".to_string(),
        "R".to_string(),
        "Points".to_string(),
        "Is valid?".to_string(),
        "Time".to_string(),
    ];
    for (reset, eps) in params {
        let reset_str = if *reset { "P" } else { "B" };
        for i in 1..5 {
            headings.push(format!("{} {} EPS={}", &headings[i], reset_str, eps));
        }
    }

    return headings;
}

pub fn random_task(ralgo_params: &[(bool, f32)]) -> io::Result<()> {
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

        let mut circles = vec![];
        for i in 0..radiuses.len() {
            circles.push(Circle::new(
                radiuses[i],
                Point {
                    x: rng.gen_range(-800.0..800.0),
                    y: rng.gen_range(-800.0..800.0),
                },
            ))
        }

        let main_circle_radius: f32 = radiuses.iter().sum();

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

    workbook.save("random-result-multi.xlsx").ok();

    Ok(())
}
