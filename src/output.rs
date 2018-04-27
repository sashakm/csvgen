use std::io
use std::io::prelude::*;
use std::fs::{OpenOptions};


fn writeOutFile(line: &str, filename: &str) -> Result<String, io::Error>
{
    OpenOptions::new()
        .append(true)
        .open(filename)?
        .writeln!(line)?;

    Ok()
}