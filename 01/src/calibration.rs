use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use std::fs::File;
use std::path::Path;

pub fn compute_calibration_value_from_file(file_path: &str) -> Result<u32> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut accumulator = 0;
    for line in reader.lines() {
        let line = line?;
        let calibration_value = get_calibration_value(&line);
        println!("{} : {}", line, calibration_value );
        accumulator += calibration_value;
        
    }
    Ok(accumulator)
}

pub fn get_calibration_value(calibrate_value: &str) -> u32 {
    let digits = extract_digits(calibrate_value);
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);
    format!("{}{}", first, last).parse().unwrap_or(0)
}

pub fn extract_digits(input: &str) -> Vec<u32> {
    input.chars()
         .filter_map(|c| c.to_digit(10))
         .filter(|&n| n >= 1 && n <= 9)
         .collect()
}

#[test]
fn test_calibrate_98() {
    assert_eq!(get_calibration_value("9h1xcrcggtwo38"), 98);
}

#[test]
fn test_calibrate_41() {
    assert_eq!(get_calibration_value("nine4pvtl"), 44);
}
#[test]
fn test_extract_digits() {
    assert_eq!(extract_digits("9h1xcrcggtwo38"), [9,1,3,8])
}

#[test]
fn test_compute_calibration_value_from_file_142(){
    assert_eq!(compute_calibration_value_from_file("./data/test.txt").unwrap(), 142);
}
