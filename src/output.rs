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
    //TODO: In a future version, open file and pass back file-handle to calller
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_and_write_outfile() {
        use output::{setup_outfile,append_work_file};
        use std::path::Path;
        use std::io::prelude::*;
        use std::fs::File;

        let outfile = setup_outfile("./foobar.csv").unwrap();
        assert!(Path::new("./foobar.csv").is_file());
        let teststring: &str = "teststring";
        append_work_file(&teststring, &outfile).expect("Could not write to file!");
        let mut testfile = File::open(&outfile).unwrap();
        let mut written_string = String::new();
        &testfile.read_to_string(&mut written_string);
        assert_eq!(&teststring, &written_string);
    }
}