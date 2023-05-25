pub fn generate_rust_calculator_files() -> (String, String) {
    let program_file_content = r#"mod test;

pub fn add_numbers(num1: &str, num2: &str) -> i32 {
    // Implement the addition logic here
    // ...
}"#
    .trim()
    .to_string();

    let test_file_content = r#"#[cfg(test)]
mod tests {
    use crate::add_numbers;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers("2", "3"), 5);
        assert_eq!(add_numbers("-5", "10"), 5);
        assert_eq!(add_numbers("0", "0"), 0);
        // Add more test cases here
    }
}"#
    .trim()
    .to_string();

    (program_file_content, test_file_content)
}

pub fn generate_rust_dsa_files() -> (String, String) {
    let program_file_content = r#"
        // TODO
    "#
    .to_string();

    let test_file_content = r#"
        // TODO
    "#
    .to_string();

    (program_file_content, test_file_content)
}

pub fn generate_rust_cargo_file(day: &str) -> String {
    format!(
        r#"[package]
name = "kata_machine_rust_calculator_{}"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
path = "lib.rs"
"#,
        day,
    )
}
