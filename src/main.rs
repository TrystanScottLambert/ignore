mod git_ignore_file_manager;

use crate::git_ignore_file_manager::{IgnoreType, write_template};

fn main() {
    write_template(IgnoreType::Python);
    write_template(IgnoreType::Rust);
}
