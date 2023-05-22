use crate::{file_utils::create_file_with_content, Kata};
use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn generate_typescript_files(day_folder_path: &PathBuf, kata: Kata) -> Result<(), String> {
    let (index_file_content, test_file_content) = match kata {
        Kata::Calculator => generate_typescript_calculator_files(),
        Kata::Dsa => generate_typescript_dsa_files(),
    };

    create_file_with_content(day_folder_path, "index.ts", &index_file_content)?;
    create_file_with_content(day_folder_path, "test.ts", &test_file_content)?;
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

fn generate_typescript_calculator_files() -> (String, String) {
    let program_file_content = r#"export function addNumbers(num1: string, num2: string): number {
    // Implement the addition logic here
    // ...
}"#
    .trim()
    .to_string();

    let test_file_content = r#"import { addNumbers } from './index';

describe('Calculator', () => {
    it('should add numbers correctly', () => {
        expect(addNumbers("2", "3")).toEqual(5);
        expect(addNumbers("-5", "10")).toEqual(5);
        expect(addNumbers("0", "0")).toEqual(0);
        // Add more test cases here
    });
});"#
        .trim()
        .to_string();

    (program_file_content, test_file_content)
}

fn generate_typescript_dsa_files() -> (String, String) {
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
