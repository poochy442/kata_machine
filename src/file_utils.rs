use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn create_file_with_content(
    file_dir: &PathBuf,
    file_name: &str,
    content: &str,
) -> Result<(), String> {
    let file_path = file_dir.join(file_name);
    if let Ok(mut file) = fs::File::create(&file_path) {
        if let Ok(_) = file.write_all(content.as_bytes()) {
            Ok(())
        } else {
            Err(format!("Error writing to file!"))
        }
    } else {
        Err(format!("Error creating file {}!", file_path.display()))
    }
}
