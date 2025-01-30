#[allow(dead_code)]
pub fn fizz_buzz(fizz: i32, buzz: i32, max: i32) -> String {
    let mut output = String::new();
    for i in 1..=max {
        output += if i % fizz == 0 { "fizz" } else { "" };
        output += if i % buzz == 0 { "buzz" } else { "" };
        if !output.ends_with('z') {
            output += &i.to_string();
        }
        output += if i != max { ", " } else { "" };
    }
    output
}
