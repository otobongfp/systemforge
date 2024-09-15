use std::fs;
// use std::io;
use std::path::Path;

pub fn analyze_file_system(path: &str) {
    let path = Path::new(path);

    if !path.exists() {
        println!("The path does not exist.");
        return;
    }

    if !path.is_dir() {
        println!("The path is not a directory.");
        return;
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let metadata = fs::metadata(entry.path()).unwrap();
                        let file_type = if metadata.is_dir() { "Directory" } else { "File" };
                        let file_size = metadata.len();
                        println!("{} - {} ({} bytes)", entry.path().display(), file_type, file_size);
                    }
                    Err(err) => {
                        println!("Error reading directory entry: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            println!("Error reading directory: {}", err);
        }
    }
}
