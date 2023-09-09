use crate::{file_utils::create_file_with_content, Kata};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use super::generation::*;

pub fn generate_typescript_files(day_folder_path: &PathBuf, kata: Kata) -> Result<(), String> {
    let src_folder_path = day_folder_path.join("src");
    if !src_folder_path.exists() {
        fs::create_dir(&src_folder_path)
            .map_err(|e| format!("Failed to create src directory: {}", e))?;
    };

    match kata {
        Kata::Calculator => generate_typescript_calculator(&src_folder_path)?,
        Kata::Dsa => generate_typescript_dsa(&src_folder_path)?,
        Kata::Blank => generate_typescript_blank(&src_folder_path)?,
    };

    let (package_json, jest_config, ts_config) = generate_typescript_config_files();
    create_file_with_content(day_folder_path, "package.json", &package_json)?;
    create_file_with_content(day_folder_path, "jest.config.js", &jest_config)?;
    create_file_with_content(day_folder_path, "tsconfig.json", &ts_config)?;

    Command::new("npm")
        .arg("i")
        .current_dir(day_folder_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to install dependencies: {}", e))?;

    Ok(())
}

fn generate_typescript_calculator(src_folder_path: &Path) -> Result<(), String> {
    let (program_file_content, test_file_content) = generate_typescript_calculator_files();

    create_file_with_content(src_folder_path, "calculator.ts", &program_file_content)?;
    create_file_with_content(src_folder_path, "calculator.spec.ts", &test_file_content)?;

    Ok(())
}

fn generate_typescript_dsa(src_folder_path: &Path) -> Result<(), String> {
    let (program_file_content, test_file_content) = generate_typescript_dsa_files();

    create_file_with_content(src_folder_path, "dsa.ts", &program_file_content)?;
    create_file_with_content(src_folder_path, "dsa.spec.ts", &test_file_content)?;

    Ok(())
}

fn generate_typescript_blank(src_folder_path: &Path) -> Result<(), String> {
    let (program_file_content, test_file_content) = generate_typescript_blank_files();

    create_file_with_content(src_folder_path, "solution.ts", &program_file_content)?;
    create_file_with_content(src_folder_path, "solution.spec.ts", &test_file_content)?;

    Ok(())
}

pub fn run_typescript_tests(day_folder_path: &Path) -> Result<(), String> {
    Command::new("npm")
        .arg("test")
        .current_dir(day_folder_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to run TypeScript tests: {}", e))?;

    Ok(())
}
