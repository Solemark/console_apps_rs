const PI: f32 = std::f32::consts::PI;

#[allow(dead_code)]
pub fn area(radius: f32) -> Result<f32, String> {
    if validate(radius) {
        Ok(PI * radius.powi(2))
    } else {
        Err(get_error())
    }
}

#[allow(dead_code)]
pub fn perimeter(radius: f32) -> Result<f32, String> {
    if validate(radius) {
        Ok(2.0 * PI * radius)
    } else {
        Err(get_error())
    }
}

fn validate(radius: f32) -> bool {
    radius >= 0.0
}

fn get_error() -> String {
    String::from("radius is smaller then 0")
}
