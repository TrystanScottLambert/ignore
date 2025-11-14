/// Module for finding appending and writing gitignore files.
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub enum IgnoreType {
    Python,
    Rust,
    R,
    Custom,
}
/// Reads the git ignore .dat files in data/
fn read_ignore_template(which: IgnoreType) -> String {
    let template_file = match which {
        IgnoreType::Python => "./src/data/python.dat",
        IgnoreType::Rust => "./src/data/rust.dat",
        IgnoreType::R => "./src/data/r.dat",
        IgnoreType::Custom => "./src/data/my_own.dat",
    };
    fs::read_to_string(template_file).expect("Core tempalates not found.")
}

pub fn write_template(which: IgnoreType) {
    let mut existing_git = fs::read_to_string(".gitignore").unwrap_or_default();
    let header = read_ignore_template(IgnoreType::Custom);
    println!("{header}");
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
