use std::io::{self, Write};
use std::fs;

pub fn write_stdout(line: &str) -> Result<usize, io::Error> {
    let stdout = io::stdout();
    let mut handle =stdout.lock();
    let written: usize = handle.write(line.as_bytes())?;
    Ok(written)
}

fn setup_work_file(filepath: &str) -> Result<&str, io::Error>
{
    fs::create_dir_all(filepath)?;
    Ok(filepath)
}

fn append_work_file(line: &str, filename: &str) -> Result<usize, io::Error>
{
    let file = fs::OpenOptions::new()
                                .append(true)
                                .open(filename)?;
    let mut handle = io::BufWriter::new(file);
    let written: usize = handle.write(line.as_bytes())?;

    Ok(written)
}