#[allow(dead_code)]
pub fn palindrome(input: String) -> bool {
    for i in 0..input.len() {
        let x = input.chars().nth(i).unwrap_or_default();
        let y = input.chars().nth(input.len() - (i + 1)).unwrap_or_default();
        if x != y {
            return false;
        }
    }
    true
}
