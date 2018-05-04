use std::io::{self, Write};
use std::fs;
use std::path::Path;

pub fn setup_outfile(filepath: &str) -> Result<&Path, io::Error>
{
    let f_path = Path::new(filepath);
    if f_path.is_file() {
        //TODO: Ask the user if wiping an existing file if exists is fine.
        panic!("Target file path already exists. Quitting.");
    }
    let _outfile: fs::File = fs::File::create(filepath)?;
    Ok(f_path)
}

pub fn write_stdout(line: &str) -> Result<usize, io::Error>
{
    let written: usize = io::stdout().write(&line.as_bytes())?;
    Ok(written)
}

pub fn append_work_file(line: &str, filename: &Path) -> Result<usize, io::Error>
{
    let file = fs::OpenOptions::new()
                                .append(true)
                                .open(filename)?;
    let mut handle = io::BufWriter::new(file);
    let written: usize = handle.write(line.as_bytes())?;

    Ok(written)
}