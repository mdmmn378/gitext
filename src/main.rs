use clap::Parser;
use ignore::WalkBuilder;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Converts a Git repo into an LLM-consumable text file
#[derive(Parser, Debug)]
#[command(name = "gitext")]
#[command(author = "Mamun")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Summarize git repo source + docs to a txt file")]
struct Args {
    /// Path to Git repo
    #[arg(short, long)]
    repo: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,

    /// Include files ignored by .gitignore
    #[arg(long)]
    no_gitignore: bool,
}

fn is_relevant_file(path: &Path) -> bool {
    let extensions = [
        "rs", "py", "go", "ts", "js", "java", "cpp", "c", "md", "txt", "toml", "json", "yaml",
        "yml", "proto",
    ];
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| extensions.contains(&ext))
        .unwrap_or(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut builder = WalkBuilder::new(&args.repo);
    builder.standard_filters(true); // ignore .git, target/, etc.
    if args.no_gitignore {
        builder.git_ignore(false);
    }

    let mut out = fs::File::create(&args.output)?;

    for result in builder.build() {
        let entry = result?;
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            let path = entry.path();
            if is_relevant_file(path) {
                writeln!(out, "\n=== File: {} ===\n", path.display())?;
                let content = fs::read_to_string(path).unwrap_or_default();
                out.write_all(content.as_bytes())?;
            }
        }
    }

    println!("✅ Summary written to {}", args.output.display());
    Ok(())
}
