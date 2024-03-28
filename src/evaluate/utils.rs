use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

use rust_xlsxwriter::{Format, Worksheet};

use crate::utils::FloatType;

fn get_buf_reader(file_name: String) -> BufReader<File> {
    let file = File::open(file_name).expect("Failed to open file");
    BufReader::new(file)
}

pub fn get_input_data(test_number: u32) -> (usize, Vec<FloatType>) {
    let file_name = format!("./input/input{:03}.txt", test_number);
    let reader = get_buf_reader(file_name);

    let mut lines = reader.lines();
    let first_line = lines.next().expect("Empty file").unwrap();
    let n: usize = first_line.trim().parse().expect("Invalid number");

    let mut radiuses: Vec<FloatType> = Vec::with_capacity(n);
    for line in lines {
        let radius: FloatType = line
            .expect("Failed to read line")
            .trim()
            .parse()
            .expect("Invalid number");

        radiuses.push(radius);
    }

    return (n, radiuses);
}

pub fn get_jury_answer(test_number: u32) -> FloatType {
    let file_name = format!("./output/out{:03}.txt", test_number);
    let reader = get_buf_reader(file_name);

    reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .to_string()
        .parse::<FloatType>()
        .expect("Valid float jury answer.")
}

pub fn write_row_block(
    worksheet: &Arc<Mutex<&mut Worksheet>>,
    row: u32,
    col: u16,
    main_circle_radius: FloatType,
    is_valid: bool,
    points: FloatType,
    time: FloatType,
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
        .write_with_format(row, col + 3, time, &format)
        .ok();
}

pub fn calculate_points(answer: FloatType, jury_answer: FloatType) -> FloatType {
    ((2.0 - (answer / jury_answer)) * 100.0).max(0.0)
}
