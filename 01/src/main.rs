fn main() {
    println!("Hello, world!");
    get_calibration_value("9h1xcrcggtwo38");
}

fn get_calibration_value(calibrate_value: &str) -> u32 {
    let digits = extract_digits(calibrate_value);
    let first = digits.first().unwrap_or(&0);
    let last = digits.last().unwrap_or(&0);
    format!("{}{}", first, last).parse().unwrap_or(0)
}

fn extract_digits(input: &str) -> Vec<u32> {
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
