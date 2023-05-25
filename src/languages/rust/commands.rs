use crate::{file_utils::create_file_with_content, Kata};
use std::{
    path::Path,
    process::{Command, Stdio},
};

use super::generation::*;

pub fn generate_rust_files(day_folder_path: &Path, kata: Kata) -> Result<(), String> {
    let (main_file_content, test_file_content) = match kata {
        Kata::Calculator => generate_rust_calculator_files(),
        Kata::Dsa => generate_rust_dsa_files(),
    };

    let day = day_folder_path.file_name().unwrap().to_string_lossy();
    let cargo_file_content = generate_rust_cargo_file(&day);

    create_file_with_content(day_folder_path, "lib.rs", &main_file_content)?;
    create_file_with_content(day_folder_path, "test.rs", &test_file_content)?;
    create_file_with_content(day_folder_path, "Cargo.toml", &cargo_file_content)?;
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
