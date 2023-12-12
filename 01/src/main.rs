mod calibration;

use std::io::Result;

fn main() -> Result<()> {
    let calibration_result = calibration::compute_calibration_value_from_file("./data/calibration.txt")?;
    println!("{}", calibration_result);
    Ok(())
}
