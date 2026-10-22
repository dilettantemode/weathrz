use clap::CommandFactory;
use clap_mangen::Man;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use weathr::cli::Cli;

fn output_path() -> PathBuf {
    std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("dist/weathrz.1"))
}

fn main() -> io::Result<()> {
    let output_path = output_path();
    let parent = output_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    fs::create_dir_all(parent)?;

    let file = File::create(&output_path)?;
    let mut writer = io::BufWriter::new(file);
    Man::new(Cli::command()).render(&mut writer)?;

    eprintln!("Generated {}", output_path.display());
    Ok(())
}
