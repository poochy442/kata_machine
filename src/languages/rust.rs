use crate::{file_utils::create_file_with_content, Kata};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn generate_rust_files(day_folder_path: &PathBuf, kata: Kata) -> Result<(), String> {
    let (main_file_content, test_file_content) = match kata {
        Kata::Calculator => generate_rust_calculator_files(),
        Kata::Dsa => generate_rust_dsa_files(),
    };

    let day = day_folder_path.file_name().unwrap().to_string_lossy();
    let cargo_file_content = format!(
        r#"[package]
name = "kata_machine_rust_calculator_{}"
version = "0.1.0"
edition = "2021"

[dependencies]

[[bin]]
name = "kata_machine_rust_calculator_day{}"
path = "main.rs"
"#,
        day, day,
    );

    create_file_with_content(day_folder_path, "main.rs", &main_file_content)?;
    create_file_with_content(day_folder_path, "test.rs", &test_file_content)?;
    create_file_with_content(day_folder_path, "cargo.toml", &cargo_file_content)?;
    Ok(())
}

pub fn run_rust_tests(day_folder_path: &Path) -> Result<(), String> {
    Command::new("cargo")
        .arg("test")
        .current_dir(day_folder_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to execute Rust tests: {}", e))?;

    Ok(())
}

fn generate_rust_calculator_files() -> (String, String) {
    let program_file_content = r#"pub fn add_numbers(num1: &str, num2: &str) -> i32 {
    // Implement the addition logic here
    // ...
}"#
    .trim()
    .to_string();

    let test_file_content = r#"#[cfg(test)]
mod tests {
    use super::*;

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

fn generate_rust_dsa_files() -> (String, String) {
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
