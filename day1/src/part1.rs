use std::fs;

fn main() {
    let input = read_data("data.txt");

    let result = sum_calibration_values(input);

    println!("{}", result);
}

fn sum_calibration_values(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|input_string| extract_calibration_value(input_string.to_owned()))
        .sum()
}

fn extract_calibration_value(input: String) -> u32 {
    let numeric_chars = input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<Vec<char>>();

    let calibration_value = format!(
        "{}{}",
        numeric_chars.first().unwrap(),
        numeric_chars.last().unwrap()
    );

    calibration_value.parse::<u32>().unwrap()
}

fn read_data(filepath: &str) -> Vec<String> {
    let data = fs::read_to_string(filepath).expect("unable to read file");
    data.lines().map(String::from).collect::<Vec<String>>()
}
