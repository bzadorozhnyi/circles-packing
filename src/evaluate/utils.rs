use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rust_xlsxwriter::{Format, Worksheet};

pub fn get_input_data(reader: BufReader<File>) -> (usize, Vec<f32>) {
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

pub fn get_jury_answer(test_number: u32) -> f32 {
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

pub fn write_row_block(
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

pub fn calculate_points(answer: f32, jury_answer: f32) -> f32 {
    ((2.0 - (answer / jury_answer)) * 100.0).max(0.0).round()
}