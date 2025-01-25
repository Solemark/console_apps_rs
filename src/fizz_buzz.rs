#[allow(dead_code)]
pub fn fizz_buzz(fizz: i32, buzz: i32, max: i32) -> String {
    let mut output = String::new();
    for i in 1..=max {
        output += &if_then_else(i % fizz == 0, "fizz".to_string());
        output += &if_then_else(i % buzz == 0, "buzz".to_string());
        output += &if_then_else(!output.ends_with('z'), i.to_string());
        output += &if_then_else(i != max, ", ".to_string());
    }
    output
}

fn if_then_else(condition: bool, success: String) -> String {
    if condition {
        success
    } else {
        String::new()
    }
}
