/// Module for finding appending and writing gitignore files.
use std::{fmt::format, fs};

enum IgnoreType {
    PYTHON,
    RUST,
    R,
    CUSTOM,
}
/// Reads the git ignore .dat files in data/
fn read_ignore_template(which: IgnoreType) -> String {
    let template_file = match which {
        IgnoreType::PYTHON => "./src/data/python.dat",
        IgnoreType::RUST => "./src/data/rust.dat",
        IgnoreType::R => "./src/data/r.dat",
        IgnoreType::CUSTOM => "./src/data/my_own.dat",
    };
    fs::read_to_string(template_file).expect("Core tempalates not found.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_template_files() {
        let rust_string = read_ignore_template(IgnoreType::RUST);
        let python_string = read_ignore_template(IgnoreType::PYTHON);
        let r_string = read_ignore_template(IgnoreType::R);
        let own_string = read_ignore_template(IgnoreType::CUSTOM);
        println!("{rust_string}");
        println!("{python_string}");
        println!("{r_string}");
        println!("{own_string}");
    }
}
