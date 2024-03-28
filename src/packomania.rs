use crate::evaluate::{
    heuristic_single_case_console::heuristic_single_case_console,
    random_single_case_console::random_single_case_console,
};

fn get_input_data(test_number: u32) -> (f64, Vec<f64>, Vec<(f64, f64)>) {
    use std::str::FromStr;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file_name = format!("./packomania/{test_number}.txt");
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().expect("Empty file").unwrap();
    let main_radius: f64 = first_line
        .trim()
        .parse()
        .expect("Invalid main circle radius");

    let mut radiuses: Vec<f64> = Vec::new();
    let mut normalized_coordinates: Vec<(f64, f64)> = Vec::new();
    for line in lines {
        let input_line = line.expect("Failed to read line");
        let mut iter = input_line.split_whitespace();

        let radius = f64::from_str(iter.next().unwrap()).unwrap();
        let x = f64::from_str(iter.next().unwrap()).unwrap();
        let y = f64::from_str(iter.next().unwrap()).unwrap();

        radiuses.push(radius);
        normalized_coordinates.push((x, y));
    }

    return (main_radius, radiuses, normalized_coordinates);
}

fn get_packomania_answer(test_number: u32) -> String {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    let file_name = format!("./packomania/{test_number}.txt");
    let file = File::open(file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    if let Some(Ok(first_line)) = reader.lines().next() {
        return first_line;
    } else {
        "".to_string()
    }
}

pub fn test_packomania_circles(test_number: u32) {
    let (main_radius, radiuses, normalized_coordinates) = get_input_data(test_number);

    let coordinates: Vec<(f64, f64)> = normalized_coordinates
        .iter()
        .map(|(x, y)| (x * main_radius, y * main_radius))
        .collect();

    for i in 0..5 {
        let (x, y) = coordinates[i];
        let temp = x.powi(2) + y.powi(2) - (main_radius - radiuses[i]).powi(2);
        if temp > 0.0 {
            println!("Circle: i = {}, {temp:1.e}", i + 1);
        }
    }

    for i in 0..5 {
        let (xi, yi) = coordinates[i];
        for j in (i + 1)..5 {
            let (xj, yj) = coordinates[j];

            let temp = -(xi - xj).powi(2) - (yi - yj).powi(2) + (radiuses[i] + radiuses[j]).powi(2);
            if temp > 0.0 {
                println!("Circles: i = {}, j = {}, {temp:1.e}", i + 1, j + 1);
            }
        }
    }
}

fn matching_decimal_digits_after_point(number1_str: &String, number2_str: &String) -> usize {
    let digits_after_point1 = number1_str.split('.').nth(1).unwrap();
    let digits_after_point2 = number2_str.split('.').nth(1).unwrap();

    let mut same_prefix_size = 0;
    for (digit1, digit2) in digits_after_point1.chars().zip(digits_after_point2.chars()) {
        if digit1 != digit2 {
            break;
        }
        same_prefix_size += 1;
    }

    same_prefix_size
}

pub fn find_best_heuristic(test_number: u32) {
    let alpha_array = [1.5, 2.0, 2.5];
    let q1_array = [0.8, 0.85, 0.9, 0.95, 1.0];
    let alpha_q1_pairs = alpha_array
        .iter()
        .flat_map(|alpha| q1_array.iter().map(|q1| (*alpha, *q1)))
        .collect::<Vec<(f64, f64)>>();

    for (index, variant) in [false, true].iter().enumerate() {
        let eps_array = [1e-5, 0.0];
        let algorithm_params = eps_array
            .iter()
            .map(|eps| (*variant, *eps))
            .collect::<Vec<(bool, f64)>>();

        let (main_circle_radiuse, mut circles) =
            heuristic_single_case_console(test_number, &algorithm_params, &alpha_q1_pairs);

        circles.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());

        println!("{} variant", index + 1);
        let packomania_answer_str = get_packomania_answer(test_number);
        let main_circle_radiuse_str = main_circle_radiuse.to_string();

        println!(
            "{}",
            matching_decimal_digits_after_point(&packomania_answer_str, &main_circle_radiuse_str)
        );
        println!("{packomania_answer_str}");
        println!("{main_circle_radiuse}");
        for circle in circles {
            println!(
                "{} {:.15} {:.15}",
                circle.radius,
                circle.center.unwrap().x,
                circle.center.unwrap().y
            );
        }
        println!();
    }
}

pub fn find_best_random(test_number: u32) {
    let alpha_array = [1.5, 2.0, 2.5];
    let q1_array = [0.8, 0.85, 0.9, 0.95, 1.0];
    let alpha_q1_pairs = alpha_array
        .iter()
        .flat_map(|alpha| q1_array.iter().map(|q1| (*alpha, *q1)))
        .collect::<Vec<(f64, f64)>>();

    for (index, variant) in [false, true].iter().enumerate() {
        let eps_array = [1e-5, 0.0];
        let algorithm_params = eps_array
            .iter()
            .map(|eps| (*variant, *eps))
            .collect::<Vec<(bool, f64)>>();

        let (main_circle_radiuse, mut circles) =
            random_single_case_console(test_number, 500, &algorithm_params, &alpha_q1_pairs);

        circles.sort_by(|a, b| a.radius.partial_cmp(&b.radius).unwrap());

        println!("{} variant", index + 1);
        let packomania_answer_str = get_packomania_answer(test_number);
        let main_circle_radiuse_str = main_circle_radiuse.to_string();

        println!(
            "{}",
            matching_decimal_digits_after_point(&packomania_answer_str, &main_circle_radiuse_str)
        );
        println!("{packomania_answer_str}");
        println!("{main_circle_radiuse}");

        for circle in circles {
            println!(
                "{} {:.15} {:.15}",
                circle.radius,
                circle.center.unwrap().x,
                circle.center.unwrap().y
            );
        }
        println!();
    }
}
