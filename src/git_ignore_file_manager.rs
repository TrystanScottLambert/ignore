/// Module for finding appending and writing gitignore files.
use std::fs;

pub enum IgnoreType {
    Python,
    Rust,
    R,
    Custom,
}
/// Reads the git ignore .dat files in data/
fn read_ignore_template(which: IgnoreType) -> String {
    match which {
        IgnoreType::Python => include_str!("data/python.dat").to_string(),
        IgnoreType::Rust => include_str!("data/rust.dat").to_string(),
        IgnoreType::R => include_str!("data/r.dat").to_string(),
        IgnoreType::Custom => include_str!("data/my_own.dat").to_string(),
    }
}

pub fn write_template(which: IgnoreType) {
    let mut existing_git = fs::read_to_string(".gitignore").unwrap_or_default();
    let header = read_ignore_template(IgnoreType::Custom);
    if !existing_git.as_str().contains(header.as_str()) {
        existing_git = format!("{header}\n{existing_git}");
    }

    let file_string = read_ignore_template(which);
    if !existing_git.as_str().contains(file_string.as_str()) {
        existing_git = format!("{existing_git}\n{file_string}");
    }
    fs::write(".gitignore", existing_git).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_template_files() {
        let rust_string = read_ignore_template(IgnoreType::Rust);
        let python_string = read_ignore_template(IgnoreType::Python);
        let r_string = read_ignore_template(IgnoreType::R);
        let own_string = read_ignore_template(IgnoreType::Custom);
        println!("{rust_string}");
        println!("{python_string}");
        println!("{r_string}");
        println!("{own_string}");
    }
}
