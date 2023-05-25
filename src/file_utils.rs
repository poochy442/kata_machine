use std::fs;
use std::io::Write;
use std::path::Path;

pub fn create_file_with_content(
    file_dir: &Path,
    file_name: &str,
    content: &str,
) -> Result<(), String> {
    let file_path = file_dir.join(file_name);
    if let Ok(mut file) = fs::File::create(&file_path) {
        file.write_all(content.as_bytes())
            .map_err(|e| format!("Error writing to file: {}", e))
    } else {
        Err(format!("Error creating file {}!", file_path.display()))
    }
}
