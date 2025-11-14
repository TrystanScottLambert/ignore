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
    let template_file = match which {
        IgnoreType::Python => "./src/data/python.dat",
        IgnoreType::Rust => "./src/data/rust.dat",
        IgnoreType::R => "./src/data/r.dat",
        IgnoreType::Custom => "./src/data/my_own.dat",
    };
    fs::read_to_string(template_file).expect("Core tempalates not found.")
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
