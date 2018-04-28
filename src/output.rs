use std::io;
use std::io::prelude::*;
use std::fs;

fn setupWorkFile(filepath: &str) -> Result<String, fs::Error>
{
    fs::create_dir_all(filepath)?;
    Ok(filepath)
}

fn writeOutFile(line: &str, filename: &str) -> Result<String, io::Error>
{
    fs::OpenOptions::new()
        .append(true)
        .open(filename)?
        .writeln!(line)?;

    Ok()
}