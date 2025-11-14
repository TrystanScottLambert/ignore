use clap::{Parser, ValueEnum};

mod git_ignore_file_manager;
use git_ignore_file_manager::{IgnoreType, write_template};

#[derive(Parser)]
#[command(name = "ignore")]
#[command(about = "Add gitignore templates to your .gitignore file", long_about = None)]
struct Cli {
    /// The type of gitignore template to add
    #[arg(value_enum)]
    ignore_type: IgnoreTypeArg,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum IgnoreTypeArg {
    Python,
    Rust,
    R,
    Custom,
}

impl From<IgnoreTypeArg> for IgnoreType {
    fn from(arg: IgnoreTypeArg) -> Self {
        match arg {
            IgnoreTypeArg::Python => IgnoreType::Python,
            IgnoreTypeArg::Rust => IgnoreType::Rust,
            IgnoreTypeArg::R => IgnoreType::R,
            IgnoreTypeArg::Custom => IgnoreType::Custom,
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let ignore_type: IgnoreType = cli.ignore_type.into();
    write_template(ignore_type);
    println!(
        "Successfully added {} template to .gitignore",
        match cli.ignore_type {
            IgnoreTypeArg::Python => "Python",
            IgnoreTypeArg::Rust => "Rust",
            IgnoreTypeArg::R => "R",
            IgnoreTypeArg::Custom => "Custom",
        }
    );
}
