use anyhow::Result;
use std::env;
use std::path::Path;

fn create_file(path: &Path) -> Result<&Path> {
    if let Some(parent) = path.parent() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => {
                // If it's not \n but "", breaks the program. TODO: Investigate this bug.
                std::fs::write(path, "\n")?;
                Ok(path)
            }
            _ => anyhow::bail!("An error occurred when creating the provided file (directories)."),
        }
    } else {
        anyhow::bail!("An error occurred when trying to parse file path parent.")
    }
}

fn is_file(path: &Path) -> bool {
    if path.ends_with("/") {
        false
    } else if path.exists() && path.is_file() {
        true
    } else { !(path.exists() && path.is_dir()) }
}

pub fn get_file_name_arg() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("File path must be provided.")
    }
    let string_path = &args[1];

    let path = Path::new(string_path);
    if !is_file(path) {
        anyhow::bail!("Provided path must be a file!")
    }
    let abs_path = std::path::absolute(path)?;
    let abs_path = match abs_path.canonicalize() {
        Ok(path) => path,
        Err(_) => create_file(&abs_path)?.to_path_buf(),
    };

    Ok(abs_path.to_string_lossy().to_string())
}
