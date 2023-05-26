use std::fs;

use clap::{Parser, Subcommand};
use kata_machine::{Kata, KataInput, Language, Session};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Generate {
        #[arg(short, long, default_value_t = String::from("rust"))]
        language: String,
        #[arg(short, long, default_value_t = String::from("calculator"))]
        kata: String,
    },
    Test {
        #[arg(short, long, default_value_t = String::from("session"))]
        language: String,
        #[arg(short, long, default_value_t = String::from("session"))]
        kata: String,
    },
    Clean,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => kata_machine::init(),
        Some(Commands::Generate { language, kata }) => {
            let (l, k) = validate_generate_input(language, kata)?;
            kata_machine::generate_kata(KataInput {
                language: l,
                kata: k,
            })
        }
        Some(Commands::Test { language, kata }) => {
            let (l, k) = validate_test_input(language, kata)?;
            kata_machine::run_tests(KataInput {
                language: l,
                kata: k,
            })
        }
        Some(Commands::Clean) => kata_machine::clean(),
        None => Err(String::from(
            "Invalid command. Available commands: init, generate, test, clean",
        )),
    }
}

fn validate_generate_input(language: &str, kata: &str) -> Result<(Language, Kata), String> {
    let l = validate_language(language)?;
    let k = validate_kata(kata)?;

    Ok((l, k))
}

fn validate_test_input(language: &str, kata: &str) -> Result<(Language, Kata), String> {
    let current_dir = std::env::current_dir()
        .map_err(|err| format!("Error getting current directory: {}", err))?;

    if language.is_empty() | kata.is_empty() | !((language == "session") | (kata == "session")) {
        let l = validate_language(language)?;
        let k = validate_kata(kata)?;
        return Ok((l, k));
    }

    let session_file_path = current_dir.join("session.json");

    let session_json = fs::read_to_string(session_file_path)
        .map_err(|err| format!("Error reading session file: {}", err))?;

    let session: Session = serde_json::from_str(&session_json)
        .map_err(|err| format!("Error parsing session JSON: {}", err))?;

    let l = Language::get(session.language).unwrap();
    let k = Kata::get(session.kata).unwrap();

    Ok((l, k))
}

fn validate_language(language: &str) -> Result<Language, String> {
    let valid_languages = ["rust", "ts", "cs"];
    if !valid_languages.contains(&language) {
        return Err(format!(
            "Invalid language: {}. Supported languages are: {}",
            language,
            valid_languages.join(", ")
        ));
    }
    Ok(Language::get(language).unwrap())
}

fn validate_kata(kata: &str) -> Result<Kata, String> {
    let valid_katas = ["calculator", "dsa"];
    if !valid_katas.contains(&kata) {
        return Err(format!(
            "Invalid kata: {}. Supported katas are: {}",
            kata,
            valid_katas.join(", ")
        ));
    }
    Ok(Kata::get(kata).unwrap())
}
