mod circle;
mod packing;
mod plot;
mod point;
mod ralgo;

use packing::find_answer;
use ralgo::dichotomy_step_ralgo;
use rust_xlsxwriter::{Format, Formula, Workbook, Worksheet};

use std::time::Instant;
use std::{fs, io};
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
    circles: &Vec<circle::Circle>,
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
        .write_with_format(
            row,
            column + 2,
            packing::is_valid_pack(main_circle_radius, &circles),
            &format,
        )
        .ok();
    worksheet
        .write_with_format(row, column + 3, time, &format)
        .ok();
}

fn calculate_points(answer: f32, jury_answer: f32) -> f32 {
    ((2.0 - (answer / jury_answer)) * 100.0).max(0.0).round()
}

fn main() -> io::Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

    // setup headings
    for (column, data) in [
        "Test",
        "R",
        "Points",
        "Is valid?",
        "Time",
        "R (ralgo)",
        "Points (ralgo)",
        "Is valid? (ralgo)",
        "Time (ralgo)",
        "R (ralgo) (2)",
        "Points (ralgo) (2)",
        "Is valid? (ralgo) (2)",
        "Time (ralgo) (2)",
        "R (ralgo) (3)",
        "Points (ralgo) (3)",
        "Is valid? (ralgo) (3)",
        "Time (ralgo) (3)",
        "R (ralgo) (4)",
        "Points (ralgo) (4)",
        "Is valid? (ralgo) (4)",
        "Time (ralgo) (4)",
    ]
    .iter()
    .enumerate()
    {
        worksheet
            .write_with_format(0, column as u16, *data, &cell_format)
            .ok();
    }

    let mut sorted_paths = fs::read_dir("./input/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    sorted_paths.sort();

    for (row, path) in sorted_paths.into_iter().enumerate() {
        println!("Test {}", row + 1);

        let file_name = path.display().to_string();
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
        let mut start = Instant::now();
        let (main_circle_radius, circles) = find_answer(&mut radiuses, 100);
        let mut elapsed = start.elapsed();
        let heuristic_time = elapsed.as_secs_f32();

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
            &circles,
            points,
            heuristic_time,
            &cell_format,
        );

        // run dichotomy ralgo with different parameters
        for (reset_step, column_start, eps) in [
            (false, 5, 0.0),
            (true, 9, 0.0),
            (false, 13, 1e-3),
            (true, 17, 1e-3),
        ] {
            start = Instant::now();
            let (new_main_circle_radius, new_circles) =
                dichotomy_step_ralgo(main_circle_radius, &circles, reset_step, eps);
            elapsed = start.elapsed();

            let points = calculate_points(new_main_circle_radius, jury_answer);

            let ralgo_time = elapsed.as_secs_f32();

            // fill in table packing result
            write_row_block(
                worksheet,
                row_number,
                column_start,
                new_main_circle_radius,
                &new_circles,
                points,
                heuristic_time + ralgo_time,
                &cell_format,
            );
        }
    }

    // calculate sum of points by heuristic packing (col=3),  total time (col=5),
    // sum of points by dichotomy step Ralgo        (col=7),  total time (col=9)
    // sum of points by dichotomy step Ralgo (2)    (col=11), total time (col=13)
    // sum of points by dichotomy step Ralgo (3)    (col=15), total time (col=17)
    // sum of points by dichotomy step Ralgo (4)    (col=19), total time (col=21)
    for col in [3, 5, 7, 9, 11, 13, 15, 17, 19, 21] {
        let column: char = (col + 64_u8) as char;
        worksheet
            .write_with_format(
                51,
                (col - 1) as u16,
                Formula::new(format!("=SUM({column}2:{column}51)")),
                &cell_format,
            )
            .ok();
    }

    worksheet.autofit();

    workbook.save("result.xlsx").ok();

    Ok(())
}
