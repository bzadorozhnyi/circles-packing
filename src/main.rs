mod circle;
mod packing;
mod plot;
mod point;
mod ralgo;

use packing::find_answer;
use ralgo::dichotomy_step_ralgo;
use rust_xlsxwriter::{Format, Formula, Workbook, Worksheet, column_number_to_name};

use std::time::Instant;
use std::{fs, thread, io};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
    column: u16,
    main_circle_radius: f32,
    is_valid: bool,
    points: f32,
    time: f32,
    format: &Format,
) {
    worksheet
        .write_with_format(row, column, main_circle_radius, &format)
        .ok();
    worksheet
        .write_with_format(row, column + 1, points, &format)
        .ok();
    worksheet
        .write_with_format(row, column + 2, is_valid, &format)
        .ok();
    worksheet
        .write_with_format(row, column + 3, time, &format)
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

fn main() -> Result<(), io::Error>{
    let total_time = Instant::now();
    let mut workbook: Workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

    let ralgo_params = [
        (false, 1e-2),
        (true, 1e-2),
        // (false, 1e-3),
        // (true, 1e-3),
        // (false, 1e-4),
        // (true, 1e-4),
        // (false, 1e-5),
        // (true, 1e-5),
        (false, 0.0),
        (true, 0.0),
    ];

    // setup headings
    for (column, data) in get_table_headings(&ralgo_params).iter().enumerate() {
        worksheet
            .write_with_format(0, column as u16, data, &cell_format)
            .ok();
    }

    let mut sorted_paths = fs::read_dir("./input/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    sorted_paths.sort();
    
    for (row, path) in sorted_paths.into_iter().enumerate() {
        println!("Test {}", row + 1);

        let file_name = path.display().to_string();
        println!("filename = {file_name}");

        let input_file = File::open(file_name).expect("Failed to open file");

        let reader = BufReader::new(input_file);

        // read input
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

        // heuristic packing
        let start = Instant::now();
        let (main_circle_radius, circles) = find_answer(&mut radiuses, 100);
        let heuristic_time = start.elapsed().as_secs_f32();

        // get jury answer of current test
        let row_number: u32 = (row + 1).try_into().unwrap();
        let jury_answer = get_jury_answer(row_number);

        let points = calculate_points(main_circle_radius, jury_answer);

        // fill in table heuristic packing result
        worksheet.write(row_number, 0, row_number).ok();
        write_row_block(
            worksheet,
            row_number,
            1,
            main_circle_radius,
            packing::is_valid_pack(main_circle_radius, &circles),
            points,
            heuristic_time,
            &cell_format,
        );

        let mut handles = vec![];

        // run dichotomy ralgo with different parameters in threads
        for (reset_step, eps) in ralgo_params {
            let copied_circles = circles.clone();

            let handle = thread::spawn(move || {
                let start = Instant::now();
                let (new_main_circle_radius, new_circles) =
                    dichotomy_step_ralgo(main_circle_radius, &copied_circles, reset_step, eps);

                let ralgo_time = start.elapsed().as_secs_f32();

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

        // write results into table
        for (index, handle) in handles.into_iter().enumerate() {
            let (radius, points, is_valid, time) = handle.join().unwrap();
            write_row_block(
                worksheet,
                row_number,
                (5 + index * 4) as u16,
                radius,
                is_valid,
                points,
                time,
                &cell_format,
            );
        }
    }

    let mut col: u16 = 2;
    while col < (ralgo_params.len() * 4 + 5) as u16 { // number of columns 
        let column: String = column_number_to_name(col);
        worksheet
            .write_with_format(
                51,
                col,
                Formula::new(format!("=SUM({column}2:{column}51)")),
                &cell_format,
            )
            .ok();
        col += 2;
    }

    worksheet.autofit();

    workbook.save("result-multi-linux.xlsx").ok();

    println!("Total time = {}", total_time.elapsed().as_secs_f32());

    Ok(())
}
