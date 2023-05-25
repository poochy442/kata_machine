use crate::{file_utils::create_file_with_content, Kata};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use super::generation::*;

pub fn generate_csharp_files(day_folder_path: &PathBuf, kata: Kata) -> Result<(), String> {
    let (program_file_content, test_file_content) = match kata {
        Kata::Calculator => generate_csharp_calculator_files(),
        Kata::Dsa => generate_csharp_dsa_files(),
    };

    let project_file_content = generate_project_file();

    create_file_with_content(day_folder_path, "Program.cs", &program_file_content)?;
    create_file_with_content(day_folder_path, "Test.cs", &test_file_content)?;
    create_file_with_content(day_folder_path, "Kata.csproj", &project_file_content)?;

    Command::new("dotnet")
        .arg("restore")
        .current_dir(day_folder_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to restore C# project: {}", e))?;

    Ok(())
}

pub fn run_csharp_tests(day_folder_path: &Path) -> Result<(), String> {
    Command::new("dotnet")
        .arg("test")
        .current_dir(day_folder_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to run C# tests: {}", e))?;

    Ok(())
}
