mod file_utils;
mod languages {
    pub mod csharp;
    pub mod rust;
    pub mod typescript;
}

use languages::{csharp, rust, typescript};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Clone, Copy)]
pub enum Language {
    Rust,
    TS,
    CSharp,
}

impl Language {
    fn as_str(&self) -> &'static str {
        match self {
            Language::CSharp => "csharp",
            Language::TS => "typescript",
            Language::Rust => "rust",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cs" => Some(Language::CSharp),
            "csharp" => Some(Language::CSharp),
            "ts" => Some(Language::TS),
            "typescript" => Some(Language::TS),
            "rust" => Some(Language::Rust),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Kata {
    Calculator,
    Dsa,
}

impl Kata {
    fn as_str(&self) -> &'static str {
        match self {
            Kata::Calculator => "calculator",
            Kata::Dsa => "dsa",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "calculator" => Some(Kata::Calculator),
            "dsa" => Some(Kata::Dsa),
            _ => None,
        }
    }
}

pub struct KataInput {
    pub language: Language,
    pub kata: Kata,
}

#[derive(Serialize, Deserialize)]
pub struct Session<'a> {
    pub language: &'a str,
    pub kata: &'a str,
    pub day: PathBuf,
}

pub fn generate_kata(input: KataInput) -> Result<(), String> {
    let current_dir = std::env::current_dir().unwrap();
    let language_dir = current_dir.join(input.language.as_str());
    let kata_dir = language_dir.join(input.kata.as_str());

    if !language_dir.exists() {
        fs::create_dir(&language_dir).unwrap();
    }
    if !kata_dir.exists() {
        fs::create_dir(&kata_dir).unwrap();
    }

    let day_folder_path = find_next_day_folder(&kata_dir);
    fs::create_dir(&day_folder_path).unwrap();

    generate_source_files(&day_folder_path, input.language, input.kata)?;

    match input.language {
        Language::Rust => link_rust_analyzer(&current_dir, &day_folder_path)?,
        _ => (),
    }

    let session = Session {
        language: input.language.as_str(),
        kata: input.kata.as_str(),
        day: day_folder_path,
    };

    let session_json = serde_json::to_string(&session)
        .map_err(|err| format!("Error serializing session to JSON: {}", err))?;
    file_utils::create_file_with_content(&current_dir, "session.json", &session_json)?;

    Ok(())
}

pub fn run_tests(kata_input: KataInput) -> Result<(), String> {
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
    let language_dir = current_dir.join(kata_input.language.as_str());
    let kata_dir = language_dir.join(kata_input.kata.as_str());

    let day_folder_path = find_most_recent_day_folder(&kata_dir).ok_or("No day folders found.")?;

    match kata_input.language {
        Language::Rust => rust::run_rust_tests(&day_folder_path),
        Language::TS => typescript::run_typescript_tests(&day_folder_path),
        Language::CSharp => csharp::run_csharp_tests(&day_folder_path),
    }
}

pub fn init() -> Result<(), String> {
    check_and_install_tool("dotnet", "dotnet-sdk-6.0")?;
    check_and_install_tool("npm", "npm")?;
    check_and_install_tool("rust", "")?;

    println!("Initialization completed successfully.");
    Ok(())
}

fn find_most_recent_day_folder(kata_dir: &PathBuf) -> Option<PathBuf> {
    let day_folders: Vec<_> = fs::read_dir(kata_dir)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.starts_with("day"))
                    .unwrap_or(false)
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if day_folders.is_empty() {
        None
    } else {
        day_folders.into_iter().max()
    }
}

fn find_next_day_folder(kata_dir: &std::path::PathBuf) -> PathBuf {
    let mut day_folder = 1;
    let mut day_folder_path = kata_dir.join(&format!("day{}", day_folder));

    while day_folder_path.exists() {
        day_folder += 1;
        day_folder_path = kata_dir.join(&format!("day{}", day_folder));
    }

    day_folder_path
}

fn generate_source_files(
    day_folder_path: &std::path::PathBuf,
    language: Language,
    kata: Kata,
) -> Result<(), String> {
    match language {
        Language::TS => typescript::generate_typescript_files(day_folder_path, kata),
        Language::Rust => rust::generate_rust_files(day_folder_path, kata),
        Language::CSharp => csharp::generate_csharp_files(day_folder_path, kata),
    }
}

fn link_rust_analyzer(current_dir: &PathBuf, day_folder_path: &PathBuf) -> Result<(), String> {
    let vscode_dir = current_dir.join(".vscode");
    if !vscode_dir.exists() {
        fs::create_dir(&vscode_dir)
            .map_err(|e| format!("Failed to create .vscode directory: {}", e))?;
    }

    let settings_path = vscode_dir.join("settings.json");
    let mut settings_content = String::new();
    let mut settings_json: Value = json!({
        "rust-analyzer.linkedProjects": [
        ]
    });

    if settings_path.exists() {
        // Read existing settings file content
        let mut file = File::open(&settings_path)
            .map_err(|e| format!("Failed to open settings file: {}", e))?;
        file.read_to_string(&mut settings_content)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;

        // Parse existing content as JSON
        settings_json = serde_json::from_str(&settings_content)
            .map_err(|e| format!("Failed to parse settings file as JSON: {}", e))?;
    }

    // Modify the array of linked projects
    let linked_projects = settings_json
        .get_mut("rust-analyzer.linkedProjects")
        .and_then(|value| value.as_array_mut())
        .ok_or("Invalid settings file format")?;

    linked_projects.push(json!("<day_folder>/Cargo.toml"));

    // Replace "<day_folder>" with actual day folder path
    settings_content = serde_json::to_string_pretty(&settings_json)
        .map_err(|e| format!("Failed to serialize settings JSON: {}", e))?;
    settings_content = settings_content.replace("<day_folder>", &day_folder_path.to_string_lossy());

    // Write the updated content back to the file
    let mut file = File::create(&settings_path)
        .map_err(|e| format!("Failed to create settings file: {}", e))?;
    file.write_all(settings_content.as_bytes())
        .map_err(|e| format!("Failed to write to settings file: {}", e))?;

    Ok(())
}

fn check_and_install_tool(tool_name: &str, package_name: &str) -> Result<(), String> {
    println!("Checking {}", tool_name);
    match tool_name {
        "rust" => check_and_install_rust(),
        _ => {
            let output = Command::new(tool_name)
                .arg("--version")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output();

            if let Ok(_) = output {
                println!("{} is already installed.", tool_name);
                Ok(())
            } else {
                println!("Installing {}...", tool_name);
                install_tool(package_name)?;
                Ok(())
            }
        }
    }
}

fn check_and_install_rust() -> Result<(), String> {
    let output = Command::new("rustc")
        .arg("--version")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        println!("rust is already installed.");
        Ok(())
    } else {
        println!("Installing rust...");
        install_rust()?;
        Ok(())
    }
}

fn install_rust() -> Result<(), String> {
    println!("Installing rust");
    let download = Command::new("curl")
        .arg("--proto")
        .arg("https")
        .arg("--tlsv1.2")
        .arg("-sSf")
        .arg("https://sh.rustup.rs")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let output = Command::new("sh")
        .stdin(Stdio::from(download.stdout.unwrap()))
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        println!("rust installation successful.");
        println!("Installing gcc...");
        install_tool("gcc")?;
        Ok(())
    } else {
        Err("Failed to install rust.".to_string())
    }
}

fn install_tool(package_name: &str) -> Result<(), String> {
    println!("Installing {}", package_name);
    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .arg(package_name)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        println!("Installation successful.");
        Ok(())
    } else {
        Err(format!("Failed to install the tool: {}", package_name))
    }
}
